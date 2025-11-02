use super::*;
use constants::*;

pub(crate) struct UpdateDataInputFieldMapper {
    unchecked: bool,
}

impl UpdateDataInputFieldMapper {
    pub fn new_checked() -> Self {
        Self { unchecked: false }
    }

    pub fn new_unchecked() -> Self {
        Self { unchecked: true }
    }
}

impl DataInputFieldMapper for UpdateDataInputFieldMapper {
    fn map_scalar<'a>(&self, ctx: &'a QuerySchema, sf: ScalarFieldRef) -> InputField<'a> {
        let base_update_type = match sf.type_identifier() {
            TypeIdentifier::Float => InputType::object(update_operations_object_type(ctx, "Float", sf.clone(), true)),
            TypeIdentifier::Decimal => {
                InputType::object(update_operations_object_type(ctx, "Decimal", sf.clone(), true))
            }
            TypeIdentifier::Int => InputType::object(update_operations_object_type(ctx, "Int", sf.clone(), true)),
            TypeIdentifier::BigInt => InputType::object(update_operations_object_type(ctx, "BigInt", sf.clone(), true)),
            TypeIdentifier::String => {
                InputType::object(update_operations_object_type(ctx, "String", sf.clone(), false))
            }
            TypeIdentifier::Boolean => InputType::object(update_operations_object_type(ctx, "Bool", sf.clone(), false)),
            TypeIdentifier::Enum(enum_id) => {
                let enum_name = ctx.internal_data_model.walk(enum_id).name();
                InputType::object(update_operations_object_type(
                    ctx,
                    &format!("Enum{enum_name}"),
                    sf.clone(),
                    false,
                ))
            }
            TypeIdentifier::Extension(_) => unreachable!("No extension field should reach this path"),
            TypeIdentifier::Json => map_scalar_input_type_for_field(ctx, &sf),
            TypeIdentifier::DateTime => {
                InputType::object(update_operations_object_type(ctx, "DateTime", sf.clone(), false))
            }
            TypeIdentifier::UUID => InputType::object(update_operations_object_type(ctx, "Uuid", sf.clone(), false)),
            TypeIdentifier::Bytes => InputType::object(update_operations_object_type(ctx, "Bytes", sf.clone(), false)),

            TypeIdentifier::Unsupported => unreachable!("No unsupported field should reach this path"),
        };

        let has_adv_json = ctx.has_capability(ConnectorCapability::AdvancedJsonNullability);
        match sf.type_identifier() {
            TypeIdentifier::Json if has_adv_json => {
                let enum_type = InputType::enum_type(json_null_input_enum(!sf.is_required()));
                let input_field = input_field(sf.name().to_owned(), vec![enum_type, base_update_type], None);

                input_field.optional()
            }

            _ => {
                let types = vec![map_scalar_input_type_for_field(ctx, &sf), base_update_type];

                let input_field = input_field(sf.name().to_owned(), types, None);
                input_field.optional().nullable_if(!sf.is_required())
            }
        }
    }

    fn map_scalar_list<'a>(&self, ctx: &'a QuerySchema, sf: ScalarFieldRef) -> InputField<'a> {
        let list_input_type = map_scalar_input_type(ctx, sf.type_identifier(), sf.is_list());
        let cloned_list_input_type = list_input_type.clone();
        let ident = Identifier::new_prisma(IdentifierType::ScalarListUpdateInput(sf.clone()));
        let type_identifier = sf.type_identifier();

        let mut input_object = init_input_object_type(ident);
        input_object.set_container(sf.container());
        input_object.set_fields(move || {
            let mut object_fields = vec![simple_input_field(operations::SET, list_input_type.clone(), None).optional()];

            if ctx.has_capability(ConnectorCapability::ScalarLists)
                && (ctx.has_capability(ConnectorCapability::EnumArrayPush) || !type_identifier.is_enum())
            {
                let map_scalar_type = map_scalar_input_type(ctx, type_identifier, false);
                object_fields.push(
                    input_field(operations::PUSH, vec![map_scalar_type, list_input_type.clone()], None).optional(),
                )
            }

            object_fields
        });
        input_object.require_exactly_one_field();

        let input_type = InputType::object(input_object);
        input_field(sf.name().to_owned(), vec![input_type, cloned_list_input_type], None).optional()
    }

    fn map_relation<'a>(&self, ctx: &'a QuerySchema, rf: RelationFieldRef) -> InputField<'a> {
        let ident = Identifier::new_prisma(IdentifierType::RelationUpdateInput(
            rf.clone(),
            rf.related_field(),
            self.unchecked,
        ));
        let rf_name = rf.name().to_owned();

        let mut input_object = init_input_object_type(ident);
        input_object.set_container(rf.related_model());
        input_object.set_fields(move || {
            let mut fields = vec![];

            if rf.related_model().supports_create_operation() {
                fields.push(input_fields::nested_create_one_input_field(ctx, rf.clone()));

                append_opt(
                    &mut fields,
                    input_fields::nested_connect_or_create_field(ctx, rf.clone()),
                );
                append_opt(&mut fields, input_fields::nested_upsert_field(ctx, rf.clone()));
                append_opt(
                    &mut fields,
                    input_fields::nested_create_many_input_field(ctx, rf.clone()),
                );
            }

            append_opt(&mut fields, input_fields::nested_set_input_field(ctx, &rf));
            append_opt(&mut fields, input_fields::nested_disconnect_input_field(ctx, &rf));
            append_opt(&mut fields, input_fields::nested_delete_input_field(ctx, &rf));

            fields.push(input_fields::nested_connect_input_field(ctx, &rf));
            fields.push(input_fields::nested_update_input_field(ctx, rf.clone()));

            append_opt(&mut fields, input_fields::nested_update_many_field(ctx, rf.clone()));
            append_opt(&mut fields, input_fields::nested_delete_many_field(ctx, &rf));
            fields
        });

        simple_input_field(rf_name, InputType::object(input_object), None).optional()
    }
}

fn update_operations_object_type<'a>(
    ctx: &'a QuerySchema,
    prefix: &str,
    sf: ScalarField,
    with_number_operators: bool,
) -> InputObjectType<'a> {
    let ident = Identifier::new_prisma(IdentifierType::FieldUpdateOperationsInput(
        !sf.is_required(),
        prefix.to_owned(),
    ));

    let mut obj = init_input_object_type(ident);
    obj.set_container(sf.container());
    obj.require_exactly_one_field();
    obj.set_fields(move || {
        let typ = map_scalar_input_type_for_field(ctx, &sf);
        let mut fields = vec![
            simple_input_field(operations::SET, typ.clone(), None)
                .optional()
                .nullable_if(!sf.is_required()),
        ];

        if with_number_operators {
            fields.push(simple_input_field(operations::INCREMENT, typ.clone(), None).optional());
            fields.push(simple_input_field(operations::DECREMENT, typ.clone(), None).optional());
            fields.push(simple_input_field(operations::MULTIPLY, typ.clone(), None).optional());
            fields.push(simple_input_field(operations::DIVIDE, typ, None).optional());
        }

        if ctx.has_capability(ConnectorCapability::UndefinedType) && !sf.is_required() {
            fields.push(simple_input_field(operations::UNSET, InputType::boolean(), None).optional());
        }

        fields
    });
    obj
}
