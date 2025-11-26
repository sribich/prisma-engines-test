use std::hash;

/// Information about a migration file within a migration directory.
#[derive(Debug, Clone)]
pub struct MigrationFile {
    /// Relative path to the migration file from the migration directory.
    /// E.g., `migration.sql`.
    pub path: String,

    /// Content of the migration file or error if it couldn't be read.
    pub content: Result<String, String>,
}

/// Information about a migration directory.
#[derive(Debug, Clone)]
pub struct MigrationDirectory {
    /// Relative path to a migration directory from `baseDir`.
    /// E.g., `20201117144659_test`.
    pub path: String,

    /// Information about the migration file within the directory.
    pub migration_file: MigrationFile,
}

impl MigrationDirectory {
    /// The `{timestamp}_{name}` formatted migration name.
    pub fn migration_name(&self) -> &str {
        self.path.as_str()
    }
}

impl hash::Hash for MigrationDirectory {
    fn hash<H: hash::Hasher>(&self, hasher: &mut H) {
        self.path.hash(hasher);
        self.migration_file.path.hash(hasher);
        if let Result::Ok(content) = &self.migration_file.content {
            content.hash(hasher);
        }
    }
}
