use json_rpc::types::DatasourceParam;

/// The type of params for the `createDatabase` method.
#[derive(Debug)]
pub struct CreateDatabaseParams {
    /// The datasource parameter.
    pub datasource: DatasourceParam,
}

/// The result for the `createDatabase` method.
#[derive(Debug)]
pub struct CreateDatabaseResult {
    /// The name of the created database.
    pub database_name: String,
}