use json_rpc::types::DatasourceParam;

/// Get the database version for error reporting.
#[derive(Debug)]
pub struct GetDatabaseVersionInput {
    /// The datasource parameter.
    pub datasource: DatasourceParam,
}
