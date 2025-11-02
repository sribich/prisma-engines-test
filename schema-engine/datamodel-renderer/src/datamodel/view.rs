use std::{borrow::Cow, fmt};

use crate::value::{Constant, Documentation, Function};

use super::{Field, IdDefinition, IndexDefinition, attributes::BlockAttribute, model::Commented};

/// Defines a model block.
#[derive(Debug)]
pub struct View<'a> {
    name: Constant<Cow<'a, str>>,
    documentation: Option<Documentation<'a>>,
    commented_out: Commented,
    ignore: Option<BlockAttribute<'a>>,
    id: Option<IdDefinition<'a>>,
    map: Option<BlockAttribute<'a>>,
    fields: Vec<Field<'a>>,
    indexes: Vec<IndexDefinition<'a>>,
    schema: Option<BlockAttribute<'a>>,
}

impl<'a> View<'a> {
    /// Create a new view declaration.
    ///
    /// ```ignore
    /// view User {
    /// //    ^^^^ name
    /// }
    /// ```
    pub fn new(name: impl Into<Cow<'a, str>>) -> Self {
        let name = Constant::new_no_validate(name.into());

        Self {
            name,
            commented_out: Commented::Off,
            map: None,
            documentation: None,
            ignore: None,
            id: None,
            schema: None,
            fields: Vec::new(),
            indexes: Vec::new(),
        }
    }

    /// Documentation of the view. If called repeatedly, adds the new docs to the end with a
    /// newline.
    ///
    /// ```ignore
    /// /// This is the documentation.
    /// view Foo {
    ///   ....
    /// }
    /// ```
    pub fn documentation(&mut self, documentation: impl Into<Cow<'a, str>>) {
        match self.documentation.as_mut() {
            Some(docs) => docs.push(documentation.into()),
            None => self.documentation = Some(Documentation(documentation.into())),
        }
    }

    /// Ignore the view.
    ///
    /// ```ignore
    /// view Foo {
    ///   @@ignore
    ///   ^^^^^^^^ this
    /// }
    /// ```
    pub fn ignore(&mut self) {
        self.ignore = Some(BlockAttribute(Function::new("ignore")));
    }

    /// Add a view-level id definition.
    ///
    /// ```ignore
    /// view Foo {
    ///   @@id([field1, field2(sort: Desc)])
    ///   ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ this
    /// }
    /// ```
    pub fn id(&mut self, id: IdDefinition<'a>) {
        self.id = Some(id);
    }

    /// Add a view-level mapping.
    ///
    /// ```ignore
    /// view Foo {
    ///   @@map("1Foo")
    ///   ^^^^^^^^^^^^^ this
    /// }
    /// ```
    pub fn map(&mut self, map: impl Into<Cow<'a, str>>) {
        let mut fun = Function::new("map");
        fun.push_param(map.into());

        self.map = Some(BlockAttribute(fun));
    }

    /// The schema attribute of the view block
    ///
    /// ```ignore
    /// view Foo {
    ///   @@schema("public")
    ///   ^^^^^^^^^^^^^^^^^^ this
    /// }
    /// ```
    pub fn schema(&mut self, schema: impl Into<Cow<'a, str>>) {
        let mut fun = Function::new("schema");
        fun.push_param(schema.into());

        self.schema = Some(BlockAttribute(fun));
    }

    /// Push a new field to the view.
    ///
    /// ```ignore
    /// view Foo {
    ///   id Int @id
    ///   ^^^^^^^^^^ this
    /// }
    /// ```
    pub fn push_field(&mut self, field: Field<'a>) {
        self.fields.push(field);
    }

    /// Push a new index to the view.
    ///
    /// ```ignore
    /// view Foo {
    ///   @@index([field1, field2])
    ///   ^^^^^^^^^^^^^^^^^^^^^^^^^ this
    /// }
    /// ```
    pub fn push_index(&mut self, index: IndexDefinition<'a>) {
        self.indexes.push(index);
    }
}

impl fmt::Display for View<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // Prefix everything with this, so if the model is commented out, so
        // is your line.
        let comment = self.commented_out;

        if let Some(ref docs) = self.documentation {
            docs.fmt(f)?;
        }

        writeln!(f, "{comment}view {} {{", self.name)?;

        for field in self.fields.iter() {
            writeln!(f, "{comment}{field}")?;
        }

        if let Some(ref id) = self.id {
            writeln!(f, "{comment}{id}")?;
        }

        for index in self.indexes.iter() {
            writeln!(f, "{comment}{index}")?;
        }

        if let Some(ref map) = self.map {
            writeln!(f, "{comment}{map}")?;
        }

        if let Some(ref ignore) = self.ignore {
            writeln!(f, "{comment}{ignore}")?;
        }

        if let Some(ref schema) = self.schema {
            writeln!(f, "{comment}{schema}")?;
        }

        writeln!(f, "{comment}}}")?;

        Ok(())
    }
}
