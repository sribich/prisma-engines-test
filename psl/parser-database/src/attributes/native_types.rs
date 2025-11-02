use crate::{ScalarFieldId, StringId, ast, context::Context};

pub(super) fn visit_model_field_native_type_attribute(
    id: ScalarFieldId,
    datasource_name: StringId,
    type_name: StringId,
    attr: &ast::Attribute,
    ctx: &mut Context<'_>,
) {
    let args = &attr.arguments;
    let args: Vec<String> = args.arguments.iter().map(|arg| arg.value.to_string()).collect();

    ctx.types[id].native_type = Some((datasource_name, type_name, args, attr.span))
}
