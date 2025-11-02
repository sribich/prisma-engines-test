use std::marker::PhantomData;

mod sealed {
    pub trait Sealed {}

    impl Sealed for super::New {}
    impl Sealed for super::Resolved {}
    impl Sealed for super::Parsed {}
    impl Sealed for super::Validated {}
}

pub trait Sealed: sealed::Sealed {}

pub struct New;
pub struct Resolved {}
pub struct Parsed {}
pub struct Validated {}

impl Sealed for New {}
impl Sealed for Resolved {}
impl Sealed for Parsed {}
impl Sealed for Validated {}

pub struct SchemaFile<T> {
    _marker: PhantomData<T>,
}

impl SchemaFile<New> {}

impl SchemaFile<Resolved> {}

impl SchemaFile<Parsed> {}

impl SchemaFile<Validated> {}
