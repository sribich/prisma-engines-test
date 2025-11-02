use crate::{Field, InternalDataModelRef, Model};
use std::fmt::Debug;
use std::hash::{Hash, Hasher};

#[derive(Clone)]
pub enum ParentContainer {
    Model(Model),
}

impl ParentContainer {
    pub fn internal_data_model(&self) -> InternalDataModelRef {
        // Unwraps are safe - the models and composites must exist after DML translation.
        match self {
            ParentContainer::Model(model) => model.dm.clone(),
        }
    }

    pub fn as_model(&self) -> Option<Model> {
        match self {
            ParentContainer::Model(m) => Some(m.clone()),
        }
    }

    pub fn name(&self) -> String {
        match self {
            ParentContainer::Model(model) => model.name().to_owned(),
        }
    }

    pub fn fields(&self) -> Vec<Field> {
        match self {
            ParentContainer::Model(model) => model.fields().all().collect(),
        }
    }

    pub fn find_field(&self, prisma_name: &str) -> Option<Field> {
        match self {
            ParentContainer::Model(model) => model.fields().find_from_all(prisma_name).ok(),
        }
    }

    pub fn is_model(&self) -> bool {
        matches!(self, Self::Model(..))
    }
}

impl From<&Model> for ParentContainer {
    fn from(model: &Model) -> Self {
        Self::Model(model.clone())
    }
}

impl From<Model> for ParentContainer {
    fn from(model: Model) -> Self {
        Self::Model(model)
    }
}

impl Debug for ParentContainer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ParentContainer::Model(m) => f
                .debug_struct("ParentContainer")
                .field("enum_variant", &"Model")
                .field("name", &m.name())
                .finish(),
        }
    }
}

impl Hash for ParentContainer {
    fn hash<H: Hasher>(&self, state: &mut H) {
        match self {
            ParentContainer::Model(model) => model.hash(state),
        }
    }
}

impl Eq for ParentContainer {}

impl PartialEq for ParentContainer {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (ParentContainer::Model(id_a), ParentContainer::Model(id_b)) => id_a == id_b,
            _ => false,
        }
    }
}
