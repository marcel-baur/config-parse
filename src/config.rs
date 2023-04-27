use config::Config;
use serde::{Deserialize, Serialize};
use log::{info, error};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Configuration {
    pub files: Vec<String>,
    pub keys: Vec<String>,
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
