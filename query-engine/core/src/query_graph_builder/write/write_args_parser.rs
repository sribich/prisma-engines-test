use super::*;
use crate::query_document::{ParsedInputMap, ParsedInputValue};
use query_structure::{
    DatasourceFieldName, Field, Model, PrismaValue, RelationFieldRef, ScalarFieldRef,
    TypeIdentifier, WriteArgs, WriteOperation,
};
use schema::constants::{args, json_null, operations};
use std::{borrow::Cow, convert::TryInto};

#[derive(Debug)]
pub struct WriteArgsParser<'a> {
    pub(crate) args: WriteArgs,
    pub(crate) nested: Vec<(RelationFieldRef, ParsedInputMap<'a>)>,
}

impl<'a> WriteArgsParser<'a> {
    /// Creates a new set of WriteArgsParser. Expects the parsed input map from the respective data key, not the enclosing map.
    /// E.g.: { data: { THIS MAP } } from the `data` argument of a write query.
    pub(crate) fn from(model: &Model, data_map: ParsedInputMap<'a>) -> QueryGraphBuilderResult<Self> {
        data_map.into_iter().try_fold(
            WriteArgsParser {
                args: WriteArgs::new_empty(crate::executor::get_request_now()),
                nested: Default::default(),
            },
            |mut args, (k, v): (Cow<'_, str>, ParsedInputValue<'_>)| {
                let field = model.fields().find_from_all(&k).unwrap();

                match field {
                    Field::Scalar(sf) if sf.is_list() => {
                        let write_op = parse_scalar_list(v)?;

                        args.args.insert(&sf, write_op);
                    }
                    Field::Scalar(sf) => {
                        let write_op: WriteOperation = parse_scalar(&sf, v)?;

                        args.args.insert(&sf, write_op)
                    }

                    Field::Relation(ref rf) => match v {
                        ParsedInputValue::Single(PrismaValue::Null) => (),
                        _ => args.nested.push((rf.clone(), v.try_into()?)),
                    },
                };

                Ok(args)
            },
        )
    }

    pub(crate) fn has_nested_operation(model: &Model, data_map: &ParsedInputMap<'a>) -> bool {
        data_map
            .iter()
            .any(|(field_name, _)| model.fields().find_from_relation_fields(field_name).is_ok())
    }
}

fn parse_scalar(sf: &ScalarFieldRef, v: ParsedInputValue<'_>) -> Result<WriteOperation, QueryGraphBuilderError> {
    match v {
        ParsedInputValue::Single(PrismaValue::Enum(e)) if sf.type_identifier() == TypeIdentifier::Json => {
            let val = match e.as_str() {
                json_null::DB_NULL => PrismaValue::Null,
                json_null::JSON_NULL => PrismaValue::Json("null".to_owned()),
                _ => unreachable!(), // Validation guarantees correct enum values.
            };

            Ok(WriteOperation::scalar_set(val))
        }
        ParsedInputValue::Single(v) => Ok(WriteOperation::scalar_set(v)),
        ParsedInputValue::Map(map) => {
            let (operation, value) = map.into_iter().next().unwrap();
            let value: PrismaValue = value.try_into()?;

            let write_op = match operation.as_ref() {
                operations::SET => WriteOperation::scalar_set(value),
                operations::UNSET => WriteOperation::scalar_unset(*value.as_boolean().unwrap()),
                operations::INCREMENT => WriteOperation::scalar_add(value),
                operations::DECREMENT => WriteOperation::scalar_substract(value),
                operations::MULTIPLY => WriteOperation::scalar_multiply(value),
                operations::DIVIDE => WriteOperation::scalar_divide(value),
                _ => unreachable!("Invalid update operation"),
            };

            Ok(write_op)
        }
        _ => unreachable!(),
    }
}

fn parse_scalar_list(v: ParsedInputValue<'_>) -> QueryGraphBuilderResult<WriteOperation> {
    match v {
        ParsedInputValue::List(_) => {
            let set_value: PrismaValue = v.try_into()?;

            Ok(WriteOperation::scalar_set(set_value))
        }
        ParsedInputValue::Map(map) => extract_scalar_list_ops(map),
        _ => unreachable!(),
    }
}

fn extract_scalar_list_ops(map: ParsedInputMap<'_>) -> QueryGraphBuilderResult<WriteOperation> {
    let (operation, value) = map.into_iter().next().unwrap();
    let pv: PrismaValue = value.try_into()?;

    match operation.as_ref() {
        operations::SET => Ok(WriteOperation::scalar_set(pv)),
        operations::PUSH => Ok(WriteOperation::scalar_add(pv)),
        _ => unreachable!("Invalid scalar list operation"),
    }
}
