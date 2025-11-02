use crate::{
    DatamodelError, ScalarFieldId, StringId,
    ast::{self, WithName},
    coerce,
    context::Context,
    generators::{CUID_SUPPORTED_VERSIONS, UUID_SUPPORTED_VERSIONS},
    types::{DefaultAttribute, ScalarFieldType, ScalarType},
};

/// @default on model scalar fields
pub(super) fn visit_model_field_default(
    scalar_field_id: ScalarFieldId,
    model_id: crate::ModelId,
    field_id: ast::FieldId,
    r#type: ScalarFieldType,
    ctx: &mut Context<'_>,
) {
    let (argument_idx, value) = match ctx.visit_default_arg_with_idx("value") {
        Ok(value) => value,
        Err(err) => return ctx.push_error(err),
    };

    let ast_model = &ctx.asts[model_id];
    let ast_field = &ast_model[field_id];

    let mapped_name = default_attribute_mapped_name(ctx);
    let default_attribute_id = ctx.current_attribute_id();

    let mut accept = move |ctx: &mut Context<'_>| {
        let default_value = DefaultAttribute {
            argument_idx,
            mapped_name,
            default_attribute: default_attribute_id,
        };

        ctx.types[scalar_field_id].default = Some(default_value);
    };

    // @default(dbgenerated(...)) is always valid.
    match value {
        ast::Expression::Function(name, funcargs, _span) if name == FN_DBGENERATED => {
            validate_dbgenerated_args(&funcargs.arguments, &mut accept, ctx);
            return;
        }
        _ => (),
    }

    match r#type {
        ScalarFieldType::Enum(enum_id) => {
            if ast_field.arity.is_list() {
                validate_enum_list_default(value, enum_id, &mut accept, ctx);
            } else {
                validate_enum_default(value, enum_id, &mut accept, ctx);
            }
        }
        ScalarFieldType::BuiltInScalar(scalar_type) => validate_model_builtin_scalar_type_default(
            scalar_field_id,
            scalar_type,
            value,
            mapped_name,
            &mut accept,
            (model_id, field_id),
            ctx,
        ),
        ScalarFieldType::Extension(_) => {
            ctx.push_attribute_validation_error("Only @default(dbgenerated(\"...\")) can be used for extension types.");
        }
        ScalarFieldType::Unsupported(_) => {
            ctx.push_attribute_validation_error(
                "Only @default(dbgenerated(\"...\")) can be used for Unsupported types.",
            );
        }
    }
}

fn validate_singular_scalar_default_literal(
    scalar_type: ScalarType,
    value: &ast::Expression,
    accept: AcceptFn<'_>,
    ctx: &mut Context<'_>,
) {
    if let ast::Expression::Array(..) = value {
        ctx.push_attribute_validation_error("The default value of a non-list field cannot be a list.")
    } else {
        validate_scalar_default_literal(scalar_type, value, accept, ctx)
    }
}

fn validate_scalar_default_literal(
    scalar_type: ScalarType,
    value: &ast::Expression,
    accept: AcceptFn<'_>,
    ctx: &mut Context<'_>,
) {
    match (scalar_type, value) {
        (ScalarType::String, ast::Expression::StringValue(_, _))
        | (ScalarType::Json, ast::Expression::StringValue(_, _))
        | (ScalarType::Bytes, ast::Expression::StringValue(_, _))
        | (ScalarType::Int, ast::Expression::NumericValue(_, _))
        | (ScalarType::BigInt, ast::Expression::NumericValue(_, _))
        | (ScalarType::Float, ast::Expression::NumericValue(_, _))
        | (ScalarType::DateTime, ast::Expression::StringValue(_, _))
        | (ScalarType::Decimal, ast::Expression::NumericValue(_, _))
        | (ScalarType::Decimal, ast::Expression::StringValue(_, _)) => accept(ctx),
        (ScalarType::Boolean, ast::Expression::ConstantValue(val, span)) => {
            validate_default_bool_value(val, *span, accept, ctx)
        }

        // Invalid scalar default.
        (scalar_type, value) => {
            validate_invalid_scalar_default(scalar_type, value, ctx);
        }
    }
}

fn validate_model_builtin_scalar_type_default(
    scalar_field_id: ScalarFieldId,
    scalar_type: ScalarType,
    value: &ast::Expression,
    mapped_name: Option<StringId>,
    accept: AcceptFn<'_>,
    field_id: (crate::ModelId, ast::FieldId),
    ctx: &mut Context<'_>,
) {
    let arity = ctx.asts[field_id.0][field_id.1].arity;
    match (scalar_type, value) {
        // Functions
        (_, ast::Expression::Function(funcname, _, _)) if funcname == FN_AUTOINCREMENT && mapped_name.is_some() => {
            ctx.push_attribute_validation_error("Naming an autoincrement default value is not allowed.")
        }
        (ScalarType::Int, ast::Expression::Function(funcname, funcargs, _))
        | (ScalarType::BigInt, ast::Expression::Function(funcname, funcargs, _))
            if funcname == FN_AUTOINCREMENT =>
        {
            validate_empty_function_args(funcname, &funcargs.arguments, accept, ctx)
        }
        (ScalarType::String, ast::Expression::Function(funcname, funcargs, _)) if funcname == FN_ULID => {
            validate_empty_function_args(funcname, &funcargs.arguments, accept, ctx)
        }
        (ScalarType::String, ast::Expression::Function(funcname, funcargs, _)) if funcname == FN_CUID => {
            validate_uid_int_args(funcname, &funcargs.arguments, &CUID_SUPPORTED_VERSIONS, accept, ctx)
        }
        (ScalarType::String, ast::Expression::Function(funcname, funcargs, _)) if funcname == FN_UUID => {
            validate_uid_int_args(funcname, &funcargs.arguments, &UUID_SUPPORTED_VERSIONS, accept, ctx)
        }
        (ScalarType::String, ast::Expression::Function(funcname, funcargs, _)) if funcname == FN_NANOID => {
            validate_nanoid_args(&funcargs.arguments, accept, ctx)
        }
        (ScalarType::DateTime, ast::Expression::Function(funcname, funcargs, _)) if funcname == FN_NOW => {
            validate_empty_function_args(FN_NOW, &funcargs.arguments, accept, ctx)
        }

        (_, ast::Expression::Function(funcname, funcargs, _)) if funcname == FN_AUTO => {
            validate_auto_args(&funcargs.arguments, accept, ctx)
        }

        (_, ast::Expression::Function(funcname, _, _)) if !KNOWN_FUNCTIONS.contains(&funcname.as_str()) => {
            ctx.types.unknown_function_defaults.push(scalar_field_id);
            accept(ctx);
        }

        // Invalid function default.
        (scalar_type, ast::Expression::Function(funcname, _, _)) => {
            validate_invalid_function_default(funcname, scalar_type, ctx);
        }

        // Scalar default literal
        (scalar_type, value) if arity.is_list() => {
            validate_builtin_scalar_list_default(scalar_type, value, accept, ctx);
        }

        (scalar_type, value) => {
            validate_singular_scalar_default_literal(scalar_type, value, accept, ctx);
        }
    }
}

fn validate_composite_builtin_scalar_type_default(
    scalar_type: ScalarType,
    value: &ast::Expression,
    accept: AcceptFn<'_>,
    field_arity: ast::FieldArity,
    ctx: &mut Context<'_>,
) {
    match (scalar_type, value) {
        // Functions
        (ScalarType::String, ast::Expression::Function(funcname, funcargs, _)) if funcname == FN_ULID => {
            validate_empty_function_args(funcname, &funcargs.arguments, accept, ctx)
        }
        (ScalarType::String, ast::Expression::Function(funcname, funcargs, _)) if funcname == FN_CUID => {
            validate_uid_int_args(funcname, &funcargs.arguments, &CUID_SUPPORTED_VERSIONS, accept, ctx)
        }
        (ScalarType::String, ast::Expression::Function(funcname, funcargs, _)) if funcname == FN_UUID => {
            validate_uid_int_args(funcname, &funcargs.arguments, &UUID_SUPPORTED_VERSIONS, accept, ctx)
        }
        (ScalarType::DateTime, ast::Expression::Function(funcname, funcargs, _)) if funcname == FN_NOW => {
            validate_empty_function_args(FN_NOW, &funcargs.arguments, accept, ctx)
        }
        (_, ast::Expression::Function(funcname, _, _)) if funcname == FN_AUTOINCREMENT || funcname == FN_AUTO => {
            ctx.push_attribute_validation_error(&format!(
                "The function `{funcname}()` is not supported on composite fields.",
            ));
        }
        (_, ast::Expression::Function(funcname, _, span)) if !KNOWN_FUNCTIONS.contains(&funcname.as_str()) => {
            ctx.push_error(DatamodelError::new_default_unknown_function(funcname, *span));
        }
        // Invalid function default.
        (scalar_type, ast::Expression::Function(funcname, _, _)) => {
            validate_invalid_function_default(funcname, scalar_type, ctx);
        }

        // Literal default
        (scalar_type, value) if field_arity.is_list() => {
            validate_builtin_scalar_list_default(scalar_type, value, accept, ctx);
        }

        (scalar_type, value) => {
            validate_singular_scalar_default_literal(scalar_type, value, accept, ctx);
        }
    }
}

fn default_attribute_mapped_name(ctx: &mut Context<'_>) -> Option<StringId> {
    match ctx
        .visit_optional_arg("map")
        .and_then(|name| coerce::string(name, ctx.diagnostics))
    {
        Some("") => {
            ctx.push_attribute_validation_error("The `map` argument cannot be an empty string.");
            None
        }
        Some(name) => Some(ctx.interner.intern(name)),
        None => None,
    }
}

fn validate_default_bool_value(bool_value: &str, span: diagnostics::Span, accept: AcceptFn<'_>, ctx: &mut Context<'_>) {
    match bool_value {
        "true" | "false" => accept(ctx),
        _ => ctx.push_error(DatamodelError::new_attribute_validation_error(
            "A boolean literal must be `true` or `false`.",
            "@default",
            span,
        )),
    }
}

fn validate_invalid_default_enum_value(enum_value: &str, ctx: &mut Context<'_>) {
    ctx.push_attribute_validation_error(&format!(
        "The defined default value `{enum_value}` is not a valid value of the enum specified for the field."
    ));
}

fn validate_invalid_default_enum_expr(bad_value: &ast::Expression, ctx: &mut Context<'_>) {
    ctx.push_attribute_validation_error(&format!("Expected an enum value, but found `{bad_value}`."))
}

fn validate_invalid_scalar_default(scalar_type: ScalarType, value: &ast::Expression, ctx: &mut Context<'_>) {
    ctx.push_attribute_validation_error(&format!(
        "Expected a {scalar_type} value, but found `{bad_value}`.",
        scalar_type = scalar_type.as_str(),
        bad_value = value
    ));
}

fn validate_invalid_function_default(fn_name: &str, scalar_type: ScalarType, ctx: &mut Context<'_>) {
    ctx.push_attribute_validation_error(&format!(
        "The function `{fn_name}()` cannot be used on fields of type `{scalar_type}`.",
        scalar_type = scalar_type.as_str()
    ));
}

fn validate_empty_function_args(fn_name: &str, args: &[ast::Argument], accept: AcceptFn<'_>, ctx: &mut Context<'_>) {
    if args.is_empty() {
        return accept(ctx);
    }

    ctx.push_attribute_validation_error(&format!(
        "The `{fn_name}` function does not take any argument. Consider changing this default to `{fn_name}()`.",
    ));
}

fn validate_auto_args(args: &[ast::Argument], accept: AcceptFn<'_>, ctx: &mut Context<'_>) {
    if !args.is_empty() {
        ctx.push_attribute_validation_error("`auto()` takes no arguments");
    } else {
        accept(ctx)
    }
}

fn validate_dbgenerated_args(args: &[ast::Argument], accept: AcceptFn<'_>, ctx: &mut Context<'_>) {
    let mut bail = || {
        // let's not mention what we don't want to see.
        ctx.push_attribute_validation_error("`dbgenerated()` takes a single String argument")
    };

    if args.len() > 1 {
        bail()
    }

    match args.first().map(|arg| &arg.value) {
        Some(ast::Expression::StringValue(val, _)) if val.is_empty() => {
            ctx.push_attribute_validation_error(
                "dbgenerated() takes either no argument, or a single nonempty string argument.",
            );
        }
        None | Some(ast::Expression::StringValue(_, _)) => accept(ctx),
        _ => bail(),
    }
}

fn format_valid_values<const N: usize>(valid_values: &[u8; N]) -> String {
    match valid_values.len() {
        0 => String::new(),
        1 => valid_values[0].to_string(),
        2 => format!("{} or {}", valid_values[0], valid_values[1]),
        _ => {
            use itertools::Itertools as _;

            let (last, rest) = valid_values.split_last().unwrap();
            let rest_str = rest.iter().map(|v| v.to_string()).join(", ");
            format!("{rest_str}, or {last}")
        }
    }
}

fn validate_uid_int_args<const N: usize>(
    fn_name: &str,
    args: &[ast::Argument],
    valid_values: &[u8; N],
    accept: AcceptFn<'_>,
    ctx: &mut Context<'_>,
) {
    let mut bail = || ctx.push_attribute_validation_error(&format!("`{fn_name}()` takes a single Int argument."));

    if args.len() > 1 {
        bail()
    }

    match args.first().map(|arg| &arg.value) {
        Some(ast::Expression::NumericValue(val, _)) => match val.parse::<u8>().ok() {
            Some(val) if valid_values.contains(&val) => accept(ctx),
            _ => {
                let valid_values_str = format_valid_values(valid_values);
                ctx.push_attribute_validation_error(&format!(
                        "`{fn_name}()` takes either no argument, or a single integer argument which is either {valid_values_str}.",
                    ));
            }
        },
        None => accept(ctx),
        _ => bail(),
    }
}

fn validate_nanoid_args(args: &[ast::Argument], accept: AcceptFn<'_>, ctx: &mut Context<'_>) {
    let mut bail = || ctx.push_attribute_validation_error("`nanoid()` takes a single Int argument.");

    if args.len() > 1 {
        bail()
    }

    match args.first().map(|arg| &arg.value) {
        Some(ast::Expression::NumericValue(val, _)) => match val.parse::<u8>().ok() {
            Some(val) if val >= 2 => accept(ctx),
            _ => {
                ctx.push_attribute_validation_error(
                    "`nanoid()` takes either no argument, or a single integer argument between 2 and 255.",
                );
            }
        },
        None => accept(ctx),
        _ => bail(),
    }
}

fn validate_enum_default(
    found_value: &ast::Expression,
    enum_id: crate::EnumId,
    accept: AcceptFn<'_>,
    ctx: &mut Context<'_>,
) {
    match found_value {
        ast::Expression::ConstantValue(enum_value, _) => {
            if ctx.asts[enum_id].values.iter().any(|v| v.name() == enum_value) {
                accept(ctx)
            } else {
                validate_invalid_default_enum_value(enum_value, ctx);
            }
        }
        bad_value => validate_invalid_default_enum_expr(bad_value, ctx),
    };
}

fn validate_enum_list_default(
    found_value: &ast::Expression,
    enum_id: crate::EnumId,
    accept: AcceptFn<'_>,
    ctx: &mut Context<'_>,
) {
    match found_value {
        ast::Expression::Array(values, _) => {
            let mut valid = true;
            let mut enum_values = values.iter();
            while let (true, Some(enum_value)) = (valid, enum_values.next()) {
                valid = false;
                validate_enum_default(
                    enum_value,
                    enum_id,
                    &mut |_| {
                        valid = true;
                    },
                    ctx,
                );
            }

            if valid {
                accept(ctx);
            }
        }
        bad_value => validate_invalid_default_enum_expr(bad_value, ctx),
    };
}

fn validate_builtin_scalar_list_default(
    scalar_type: ScalarType,
    found_value: &ast::Expression,
    accept: AcceptFn<'_>,
    ctx: &mut Context<'_>,
) {
    match found_value {
        ast::Expression::Array(values, _) => {
            let mut valid = true;
            let mut values = values.iter();
            while let (true, Some(value)) = (valid, values.next()) {
                valid = false;
                validate_scalar_default_literal(
                    scalar_type,
                    value,
                    &mut |_| {
                        valid = true;
                    },
                    ctx,
                );
            }

            if valid {
                accept(ctx)
            }
        }
        _bad_value => ctx.push_attribute_validation_error("The default value of a list field must be a list."),
    }
}

const FN_AUTOINCREMENT: &str = "autoincrement";
const FN_CUID: &str = "cuid";
const FN_ULID: &str = "ulid";
const FN_DBGENERATED: &str = "dbgenerated";
const FN_NANOID: &str = "nanoid";
const FN_NOW: &str = "now";
const FN_UUID: &str = "uuid";
const FN_AUTO: &str = "auto";

const KNOWN_FUNCTIONS: &[&str] = &[
    FN_AUTOINCREMENT,
    FN_ULID,
    FN_CUID,
    FN_DBGENERATED,
    FN_NANOID,
    FN_NOW,
    FN_UUID,
    FN_AUTO,
];

type AcceptFn<'a> = &'a mut dyn for<'b, 'c> FnMut(&'b mut Context<'c>);
