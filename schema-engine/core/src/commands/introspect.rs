use json_rpc::types::SchemasContainer;

/// Introspect the database (db pull)
#[derive(Debug)]
pub struct IntrospectParams {
    /// Prisma schema files.
    pub schema: SchemasContainer,
    /// Base directory path.
    pub base_directory_path: String,
    /// Force flag.
    pub force: bool,
    /// Optional namespaces.
    pub namespaces: Option<Vec<String>>,
}

/// Result type for the introspect method.
#[derive(Debug)]
pub struct IntrospectResult {
    /// The introspected schema.
    pub schema: SchemasContainer,
    /// Optional views.
    pub views: Option<Vec<IntrospectionView>>,
    /// Optional warnings.
    pub warnings: Option<String>,
}

/// Information about a database view.
#[derive(Debug)]
pub struct IntrospectionView {
    /// The view definition.
    pub definition: String,
    /// The view name.
    pub name: String,
    /// The schema name.
    pub schema: String,
}
