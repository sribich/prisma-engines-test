//! API type definitions used by the JSON-RPC methods.

pub use crate::{
    migration_directory::{MigrationDirectory, MigrationFile},
};

// ---- Common type definitions ----

/// Information about a migration lockfile.
#[derive(Debug)]
pub struct MigrationLockfile {
    /// Relative path to the lockfile from base directory.
    /// E.g., `./migration_lock.toml`.
    pub path: String,

    /// Content of the lockfile, if it exists.
    pub content: Option<String>,
}

/// A list of migration directories with related information.
#[derive(Debug)]
pub struct MigrationList {
    /// Absolute path to the base directory of Prisma migrations.
    /// E.g., `/usr/src/app/prisma/migrations`.
    pub base_dir: String,

    /// Description of the lockfile, which may or may not exist.
    pub lockfile: MigrationLockfile,

    /// An init script that will be run on the shadow database before the migrations are applied. Can be a no-op.
    pub shadow_db_init_script: String,

    /// List of migration directories.
    pub migration_directories: Vec<MigrationDirectory>,
}

/// An object with a `url` field.
/// @deprecated
#[derive(Debug)]
pub struct UrlContainer {
    /// The URL string.
    pub url: String,
}

/// A container that holds the path and the content of a Prisma schema file.
#[derive(Debug)]
pub struct SchemaContainer {
    /// The content of the Prisma schema file.
    pub content: String,

    /// The file name of the Prisma schema file.
    pub path: String,
}

/// A container that holds multiple Prisma schema files.
#[derive(Debug)]
pub struct SchemasContainer {
    /// List of schema files.
    pub files: Vec<SchemaContainer>,
}

/// A list of Prisma schema files with a config directory.
#[derive(Debug)]
pub struct SchemasWithConfigDir {
    /// A list of Prisma schema files.
    pub files: Vec<SchemaContainer>,

    /// An optional directory containing the config files such as SSL certificates.
    pub config_dir: String,
}

/// The path to a live database taken as input. For flexibility, this can be Prisma schemas as strings, or only the
/// connection string. See variants.
#[derive(Debug)]
pub enum DatasourceParam {
    /// Prisma schema as input
    Schema(SchemasContainer),

    /// Connection string as input
    ConnectionString(UrlContainer),
}

/// Fields for the DatabaseIsBehind variant.
#[derive(Debug)]
pub struct DatabaseIsBehindFields {}
