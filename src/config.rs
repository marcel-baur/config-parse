use config::Config;
use serde::{Deserialize, Serialize};
use log::info;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Configuration {
    pub files: Vec<String>,
    pub keys: Vec<String>,
    pub filetype: String,
}

pub fn get_config() -> Option<Configuration> {
    let settings = Config::builder()
        .add_source(config::File::with_name("config_parser"))
        .add_source(config::Environment::with_prefix("APP"))
        .build()
        .unwrap();

    let conf = settings.try_deserialize::<Configuration>().unwrap();
    info!("Config: {:?}", &conf);
    Some(conf)
}
