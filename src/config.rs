use config::Config;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Configuration {
    pub file: String,
    pub keys: Vec<String>,
    pub dest: String,
}

pub fn get_config() -> Configuration {
    let settings = Config::builder()
        .add_source(config::File::with_name("config_parser"))
        .add_source(config::Environment::with_prefix("APP"))
        .build()
        .unwrap();

    let conf = settings.try_deserialize::<Configuration>().unwrap();
    println!("Config: {:?}", &conf);
    conf
}
