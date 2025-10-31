use std::sync::Arc;

#[test]
fn empty_datamodel_loads() {
    let schema = "";
    let parsed_schema = psl::parse_schema_without_extensions(schema).unwrap();
    let schema = schema::build(Arc::new(parsed_schema), true);
}
