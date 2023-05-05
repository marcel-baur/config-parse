use clap::Parser;
use config::Config;
use serde::{Deserialize, Serialize};
use tracing::{info, error};

#[derive(Clone, Debug, Serialize, Deserialize, Parser)]
#[command(author, version, about, long_about = None)]
pub struct Configuration {
    #[arg(long)]
    pub files: Vec<String>,
    #[arg(long)]
    pub keys: Vec<String>,
    #[arg(long)]
    pub cli: bool,
}

pub fn get_config() -> Option<Configuration> {
    let settings = match Config::builder()
        .add_source(config::File::with_name("config_parser"))
        .add_source(config::Environment::with_prefix("APP"))
        .build() {
            Ok(settings) => settings,
            Err(e) => {
                error!("Panic while fetching config: {:?}", e);
                return None;
            }
        };

    let conf = settings.try_deserialize::<Configuration>().unwrap();
    info!("Config: {:?}", &conf);
    Some(conf)
}
