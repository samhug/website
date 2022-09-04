use figment::{
    providers::{Env, Serialized},
    Figment,
};
use serde::{Deserialize, Serialize};
use std::{net::SocketAddr, path::PathBuf};

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Config {
    pub static_files_dir: PathBuf,
    pub listen_addr: SocketAddr,
    pub https_redirect: bool,
    pub host_redirect: Option<String>,
}

impl Config {
    pub fn from_default_sources() -> Result<Self, figment::Error> {
        let mut figment = Figment::new()
            .join(Serialized::default("listen_addr", "127.0.0.1:8000"))
            .join(Serialized::default("https_redirect", false))
            .join(Serialized::default("host_redirect", Option::<String>::None))
            // #
            ;

        // For convenience, when compiled using the debug profile,
        // serve static files from the project dir by default
        #[cfg(debug_assertions)]
        {
            figment = figment.join(Serialized::default(
                "static_files_dir",
                concat!(env!("CARGO_MANIFEST_DIR"), "/static"),
            ));
        }

        // Add configuration from environment variables
        figment = figment.merge(Env::prefixed("APP_"));

        figment.extract()
    }
}
