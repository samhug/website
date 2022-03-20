use figment::{
    providers::{Env, Serialized},
    util::map,
    Figment,
};
use serde::{Deserialize, Serialize};
use std::{net::SocketAddr, path::PathBuf};

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Config {
    pub static_files_dir: PathBuf,
    pub listen_addr: SocketAddr,
    pub http_host: Option<String>,
}

impl Config {
    pub fn from_default_sources() -> Result<Self, figment::Error> {
        let defaults = map![
            "listen_addr" => "0.0.0.0:8000",
            "static_files_dir" => "/www/public",
        ];
        Figment::from(Serialized::from(&defaults, "default"))
            .merge(Env::prefixed("APP_"))
            .extract()
    }
}
