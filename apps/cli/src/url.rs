use std::collections::HashMap;

use url::Url;

struct DatabaseConnectionParams {
    connector: String,
    user: Option<String>,
    password: Option<String>,
    host: Option<String>,
    port: Option<u16>,
}

pub fn parse_connection_string(url: &str) -> DatabaseConnectionParams {
    let uri = Url::parse(url).unwrap();

    let connector = scheme_to_connector_type(uri.scheme());

    let query = uri.query_pairs().collect::<HashMap<_, _>>();

    let user = uri.username();

    DatabaseConnectionParams {
        connector: connector.to_owned(),
        user: if user.is_empty() { None } else { Some(user.to_owned()) },
        password: uri.password().map(str::to_owned),
        host: uri.host().map(|it| it.to_string()),
        port: uri.port(),
    }
}

/// Converts a URI scheme into a known connector type.
fn scheme_to_connector_type(scheme: &str) -> &'static str {
    match scheme {
        "postgres" | "postgresql" => "postgresql",
        "mysql" => "mysql",
        "file" => "sqlite",
        _ => panic!("Unknown scheme: {}", scheme),
    }
}

pub fn pretty_connector() {}
