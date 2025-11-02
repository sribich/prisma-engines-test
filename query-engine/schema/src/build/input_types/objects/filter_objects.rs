use super::*;
use constants::filters;
use query_structure::{prelude::ParentContainer};

pub(crate) fn scalar_filter_object_type(
    ctx: &'_ QuerySchema,
    model: Model,
    include_aggregates: bool,
) -> InputObjectType<'_> {
    let ident = Identifier::new_prisma(IdentifierType::ScalarFilterInput(model.clone(), include_aggregates));

    let mut input_object = init_input_object_type(ident);
    input_object.set_container(model.clone());
    input_object.set_tag(ObjectTag::WhereInputType(ParentContainer::Model(model.clone())));
    input_object.set_fields(move || {
        let object_type = InputType::object(scalar_filter_object_type(ctx, model.clone(), include_aggregates));

        let mut input_fields = vec![
            input_field(
                filters::AND,
                vec![object_type.clone(), InputType::list(object_type.clone())],
                None,
            )
            .optional(),
            input_field(filters::OR, vec![InputType::list(object_type.clone())], None).optional(),
            input_field(
                filters::NOT,
                vec![object_type.clone(), InputType::list(object_type)],
                None,
            )
            .optional(),
        ];

        input_fields.extend(model.fields().all().filter_map(|f| match f {
            ModelField::Scalar(_) => Some(input_fields::filter_input_field(ctx, f, include_aggregates)),
            ModelField::Relation(_) => None,
        }));
        input_fields
    });

    input_object
}

pub(crate) fn where_object_type(ctx: &'_ QuerySchema, container: ParentContainer) -> InputObjectType<'_> {
    let ident = Identifier::new_prisma(IdentifierType::WhereInput(container.clone()));

    let mut input_object = init_input_object_type(ident);
    input_object.set_container(container.clone());
    input_object.set_tag(ObjectTag::WhereInputType(container.clone()));
    input_object.set_fields(move || {
        let object_type = InputType::object(where_object_type(ctx, container.clone()));

        let mut fields = vec![
            input_field(
                filters::AND,
                vec![object_type.clone(), InputType::list(object_type.clone())],
                None,
            )
            .optional(),
            input_field(filters::OR, vec![InputType::list(object_type.clone())], None).optional(),
            input_field(
                filters::NOT,
                vec![object_type.clone(), InputType::list(object_type)],
                None,
            )
            .optional(),
        ];

        let input_fields = container
            .fields()
            .into_iter()
            .map(|f| input_fields::filter_input_field(ctx, f, false));

        fields.extend(input_fields);
        fields
    });
    input_object
}

pub(crate) fn where_unique_object_type(ctx: &'_ QuerySchema, model: Model) -> InputObjectType<'_> {
    let ident = Identifier::new_prisma(IdentifierType::WhereUniqueInput(model.clone()));

    let mut input_object = init_input_object_type(ident);
    input_object.set_container(model.clone());
    input_object.set_tag(ObjectTag::WhereInputType(ParentContainer::Model(model.clone())));

    // Concatenated list of uniques/@@unique/@@id fields on which the input type constraints should be applied (that at least one of them is set).
    let constrained_fields: Vec<_> = {
        let walker = ctx.internal_data_model.walk(model.id);

        walker
            .primary_key()
            .map(compound_id_field_name)
            .into_iter()
            .chain(
                walker
                    .indexes()
                    .filter(|idx| idx.is_unique())
                    .map(compound_index_field_name),
            )
            .collect()
    };

    input_object.require_at_least_one_field();
    input_object.apply_constraints_on_fields(constrained_fields);

    input_object.set_fields(move || {
        // Split unique & ID fields vs all the other fields
        let (unique_fields, rest_fields): (Vec<_>, Vec<_>) =
            model.fields().all().partition(|f| f.is_scalar() && f.is_unique());
        // @@unique compound fields.
        let compound_uniques: Vec<_> = ctx
            .internal_data_model
            .walk(model.id)
            .indexes()
            .filter(|idx| idx.is_unique())
            .filter(|index| index.fields().len() > 1)
            .filter(|index| !index.fields().any(|f| f.is_unsupported()))
            .map(|index| {
                let fields = index
                    .fields()
                    .map(|f| ScalarFieldRef::from((model.dm.clone(), f)))
                    .collect();
                let typ = compound_field_unique_object_type(ctx, &model, index.name(), fields);
                let name = compound_index_field_name(index);

                (name, typ)
            })
            .collect();
        // @@id compound field (there can be only one per model).
        let compound_id = ctx
            .internal_data_model
            .walk(model.id)
            .primary_key()
            .filter(|pk| pk.fields().len() > 1)
            .filter(|pk| !pk.fields().any(|f| f.is_unsupported()))
            .map(|pk| {
                let name = compound_id_field_name(pk);
                let fields = model.fields().id_fields().unwrap().collect();
                let typ = compound_field_unique_object_type(ctx, &model, pk.name(), fields);

                (name, typ)
            });
        let mut fields: Vec<InputField<'_>> = unique_fields
            .into_iter()
            .map(|f| {
                let sf = f.as_scalar().unwrap();
                let name = sf.borrowed_name(&ctx.internal_data_model.schema);
                let typ = map_scalar_input_type_for_field(ctx, sf);

                simple_input_field(name, typ, None).optional()
            })
            .collect();

        // @@id compound field (there can be only one per model).
        let compound_id_field = compound_id
            .as_ref()
            .map(|(name, typ)| simple_input_field(name.clone(), InputType::object(typ.clone()), None).optional());

        // Boolean operators AND/OR/NOT, which are _not_ where unique inputs
        let where_input_type = InputType::object(where_object_type(ctx, ParentContainer::Model(model.clone())));
        let boolean_operators = vec![
            input_field(
                filters::AND,
                vec![where_input_type.clone(), InputType::list(where_input_type.clone())],
                None,
            )
            .optional(),
            input_field(filters::OR, vec![InputType::list(where_input_type.clone())], None).optional(),
            input_field(
                filters::NOT,
                vec![where_input_type.clone(), InputType::list(where_input_type)],
                None,
            )
            .optional(),
        ];

        // @@unique compound fields.
        fields.extend(
            compound_uniques
                .iter()
                .map(|(name, typ)| simple_input_field(name.clone(), InputType::object(typ.clone()), None).optional()),
        );
        fields.extend(compound_id_field);

        fields.extend(boolean_operators);
        fields.extend(
            rest_fields
                .into_iter()
                .map(|f| input_fields::filter_input_field(ctx, f, false)),
        );

        assert!(!fields.is_empty(), "where objects cannot be empty");

        fields
    });

    input_object
}

/// Generates an input object type for a compound field.
fn compound_field_unique_object_type<'a>(
    ctx: &'a QuerySchema,
    model: &Model,
    alias: Option<&str>,
    from_fields: Vec<ScalarFieldRef>,
) -> InputObjectType<'a> {
    let ident = Identifier::new_prisma(format!(
        "{}{}CompoundUniqueInput",
        model.name(),
        compound_object_name(alias, &from_fields)
    ));

    let mut input_object = init_input_object_type(ident);
    input_object.set_container(model.clone());
    input_object.set_fields(move || {
        from_fields
            .clone()
            .into_iter()
            .map(|field| {
                let name = field.name().to_owned();
                let typ = map_scalar_input_type_for_field(ctx, &field);

                simple_input_field(name, typ, None)
            })
            .collect()
    });
    input_object
}
