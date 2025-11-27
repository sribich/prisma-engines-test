mod schema;
mod file;

use anyhow::{anyhow, Result};
pub use schema::{SchemaContext, SchemaParser, SchemaValidator};
pub use file::{SchemaFile, New, Parsed, Validated};

use std::{
    convert::identity,
    env::current_dir,
    path::{Path, PathBuf},
};












const SCHEMA_PATHS: [&str; 1] = ["prisma/schema.prisma"];

#[derive(Debug)]
pub struct SchemaFiles {
    pub root_dir: PathBuf,
    pub schemas: Vec<PathBuf>,
}






/// Attempt to locate a schema context.
///
pub fn load_schema_context(cwd: Option<PathBuf>, schema: Option<PathBuf>) -> Result<SchemaFiles> {
    // TODO(sr): We should return our own error here.
    let cwd = cwd.unwrap_or(current_dir()?);

    if let Some(path) = schema {
        return load_path(&cwd, path);
    }

    for path in SCHEMA_PATHS {
        if let Ok(files) = load_path(&cwd, path.into()) {
            return Ok(files);
        }
    }

    Err(anyhow!("Failed to locate prisma schema file"))
}

fn load_path(cwd: &Path, path: PathBuf) -> Result<SchemaFiles> {
    let path = cwd.join(path);

    if !path.try_exists().is_ok_and(identity) {
        return Err(anyhow!("Path {:?} does not exist", path));
    }

    match path.is_file() {
        true => load_file(path).ok_or(anyhow!("TODO")),
        false => load_dir(path).ok_or(anyhow!("TODO")),
    }
}

fn load_file(path: PathBuf) -> Option<SchemaFiles> {
    assert!(path.is_file());

    // TODO: Error
    if path.extension().unwrap() != "prisma" {
        return None;
    }

    load_dir(path.parent().unwrap().to_owned())
}

fn load_dir(path: PathBuf) -> Option<SchemaFiles> {
    assert!(path.is_dir());

    let mut files = SchemaFiles {
        root_dir: path.clone().canonicalize().unwrap(),
        schemas: vec![],
    };

    for file in path.read_dir().unwrap() {
        let file = file.unwrap();
        let meta = file.metadata().unwrap();

        if meta.is_file() && file.path().extension().is_some_and(|it| it == "prisma") {
            files.schemas.push(file.path().canonicalize().unwrap());
        }

        if meta.is_dir()
            && let Some(inner) = load_dir(file.path())
        {
            files.schemas.extend(inner.schemas);
        }
    }

    if files.schemas.is_empty() { None } else { Some(files) }
}
