use json_rpc::types::DatasourceParam;

/// Make sure the schema engine can connect to the database from the Prisma schema.
#[derive(Debug)]
pub struct EnsureConnectionValidityParams {
    /// The datasource parameter.
    pub datasource: DatasourceParam,
}

/// Result type for the ensureConnectionValidity method.
#[derive(Debug)]
pub struct EnsureConnectionValidityResult {}