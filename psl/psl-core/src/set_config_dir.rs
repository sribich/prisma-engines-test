use connection_string::JdbcString;
use std::{borrow::Cow, collections::BTreeMap};

use crate::datamodel_connector::Flavour;

pub fn set_config_dir<'a>(flavour: Flavour, config_dir: &std::path::Path, url: &'a str) -> Cow<'a, str> {
    match flavour {
        Flavour::Sqlite => set_config_dir_sqlite(config_dir, url),
        _ => set_config_dir_default(config_dir, url),
    }
}

fn set_config_dir_default<'a>(config_dir: &std::path::Path, url: &'a str) -> Cow<'a, str> {
    let set_root = |path: &str| {
        let path = std::path::Path::new(path);

        if path.is_relative() {
            Some(config_dir.join(path).to_str().map(ToString::to_string).unwrap())
        } else {
            None
        }
    };

    let mut url = match url::Url::parse(url) {
        Ok(url) => url,
        Err(_) => return Cow::from(url), // bail
    };

    let mut params: BTreeMap<String, String> = url.query_pairs().map(|(k, v)| (k.to_string(), v.to_string())).collect();

    url.query_pairs_mut().clear();

    // Only for PostgreSQL + MySQL
    if let Some(path) = params.get("sslcert").map(|s| s.as_str()).and_then(set_root) {
        params.insert("sslcert".into(), path);
    }

    // Only for PostgreSQL + MySQL
    if let Some(path) = params.get("sslidentity").map(|s| s.as_str()).and_then(set_root) {
        params.insert("sslidentity".into(), path);
    }

    for (k, v) in params.into_iter() {
        url.query_pairs_mut().append_pair(&k, &v);
    }

    url.to_string().into()
}

pub fn set_config_dir_sqlite<'a>(config_dir: &std::path::Path, url: &'a str) -> Cow<'a, str> {
    let set_root = |path: &str| {
        let path = std::path::Path::new(path);

        if path.is_relative() {
            Some(config_dir.join(path).to_str().map(ToString::to_string).unwrap())
        } else {
            None
        }
    };

    if let Some(path) = set_root(url.trim_start_matches("file:")) {
        return Cow::Owned(format!("file:{path}"));
    };

    Cow::Borrowed(url)
}
