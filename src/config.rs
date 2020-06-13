use structopt::StructOpt;
use serde::{Deserialize, Serialize};

#[derive(Debug, StructOpt)]
pub enum ConfigOpts {
    /// Get
    Get {
        key: String,
    },
    /// Set
    Set {
        key: String,
        value: Option<String>,
    },
    /// View
    View,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    pub consumer_key: Option<String>,
    pub access_token: Option<String>,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            consumer_key: None,
            access_token: None,
        }
    }
}

const CFG_NAME: &str = env!("CARGO_PKG_NAME");

pub fn load() -> Config {
    confy::load(CFG_NAME).unwrap()
}

fn store(cfg: Config) {
    confy::store(CFG_NAME, cfg).unwrap();
}

pub fn handle(opts: &ConfigOpts, mut writer: impl std::io::Write) {
    let mut cfg = load();

    match opts {
        ConfigOpts::Get { key } => {
            let value = match key.as_str() {
                "consumer_key" => cfg.consumer_key,
                "access_token" => cfg.access_token,
                _ => panic!(format!("Invalid key: `{}`", key)),
            }.unwrap_or_default();
            writeln!(writer, "{}", value).unwrap();
        },
        ConfigOpts::Set { key, value } => {
            match key.as_str() {
                "consumer_key" => cfg.consumer_key = value.clone(),
                "access_token" => cfg.access_token = value.clone(),
                _ => panic!(format!("Invalid key: `{}`", key)),
            };
            store(cfg);
            writeln!(writer, "Success").unwrap();
        },
        ConfigOpts::View => {
            writeln!(writer, "{:?}", cfg).unwrap();
        },
    }
}