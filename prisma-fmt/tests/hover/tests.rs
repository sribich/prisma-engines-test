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
    enum_from_block_name
    enum_from_field_type
    field_from_model_field_name
    model_from_block_name
    model_from_model_type_includes_broken_relations
    model_from_model_type_on_broken_relations
    one_to_many_self_relation
    value_from_enum_value_name

}
