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
        let mut defaults = map![
            "listen_addr" => "127.0.0.1:8000",
        ];

        #[cfg(debug_assertions)]
        {
            defaults
                .insert(
                    "static_files_dir",
                    concat!(env!("CARGO_MANIFEST_DIR"), "/static"),
                )
                .unwrap();
        }

        Figment::from(Serialized::from(&defaults, "default"))
            .merge(Env::prefixed("APP_"))
            .extract()
    }
}
