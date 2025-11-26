use json_rpc::types::{SchemasWithConfigDir, UrlContainer};

/// The type of params accepted by dbExecute.
#[derive(Debug)]
pub struct DbExecuteParams {
    /// The location of the live database to connect to.
    pub datasource_type: DbExecuteDatasourceType,

    /// The input script.
    pub script: String,
}

/// The type of results returned by dbExecute.
#[derive(Debug)]
pub struct DbExecuteResult {}

/// The location of the live database to connect to.
#[derive(Debug)]
pub enum DbExecuteDatasourceType {
    /// Prisma schema files and content to take the datasource URL from.
    Schema(SchemasWithConfigDir),

    /// The URL of the database to run the command on.
    Url(UrlContainer),
}
