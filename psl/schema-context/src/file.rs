use std::{fs::read_to_string, marker::PhantomData, path::{Path, PathBuf}};

mod sealed {
    pub trait Sealed {}

    impl Sealed for super::New {}
    impl Sealed for super::Resolved {}
    impl<T> Sealed for super::Parsed<T> {}
    impl<T> Sealed for super::Validated<T> {}
}

pub trait Sealed: sealed::Sealed {}

#[derive(Clone, Debug)]
pub struct New;

#[derive(Clone, Debug)]
pub struct Resolved {}

#[derive(Clone, Debug)]
pub struct Parsed<T> {
    pub inner: T,
}

#[derive(Clone, Debug)]
pub struct Validated<T> {
    pub inner: T,
}

impl Sealed for New {}
impl Sealed for Resolved {}
impl<T> Sealed for Parsed<T> {}
impl<T> Sealed for Validated<T> {}

#[derive(Clone, Debug)]
pub struct SchemaFile<T = New> 
where
    T: Clone,
{
    path: PathBuf,
    content: String,
    context: T,
    _marker: PhantomData<T>,
}

impl<T> SchemaFile<T> 
where 
    T: Clone 
{
    pub fn content(&self) -> &str {
        &self.content        
    }

    pub fn path(&self) -> &Path {
        &self.path        
    }

    pub fn convert<P>(self, data: P) -> SchemaFile<P> 
    where
        P: Clone
    {
        SchemaFile { 
            path: self.path, 
            content: self.content, 
            context: data,
            _marker: PhantomData,
        }
    }
}

impl SchemaFile<New> {
    pub fn new(path: PathBuf) -> Self {
        let content = read_to_string(&path).unwrap();

        Self {
            path,
            content,
            context: New,
            _marker: PhantomData,
        }
    }
}

impl SchemaFile<Resolved> {}

impl<T> SchemaFile<Parsed<T>> 
where
    T: Clone
{}

impl<T> SchemaFile<Validated<T>> 
where
    T: Clone
    {}
