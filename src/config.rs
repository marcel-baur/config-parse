use config::Config;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct LoadedConfiguration {
    pub files: Vec<String>,
    pub keys: Vec<String>,
    pub filetype: String,
}

#[derive(Debug, Serialize, Deserialize)]
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

    let conf = settings.try_deserialize::<LoadedConfiguration>().unwrap();
    println!("Config: {:?}", &conf);
    Some(Configuration {
        files: conf.files,
        keys: conf.keys,
        filetype: conf.filetype,
    })
}
