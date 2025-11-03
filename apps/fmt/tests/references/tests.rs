use super::test_api::test_scenario;

macro_rules! scenarios {
    ($($scenario_name:ident)+) => {
        $(
            #[test]
            fn $scenario_name() {
                test_scenario(stringify!($scenario_name))
            }
        )*
    }
}

scenarios! {
    enum_as_type
    enum_name
    model_as_type
    model_field_name
    model_name
    model_index_fields
    model_relation_fields
    model_relation_references
    model_unique_fields
    datasource_as_attribute
    datasource_name
}
