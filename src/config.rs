use config::Config;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct LoadedConfiguration {
    pub file: String,
    pub keys: Vec<String>,
    pub dest: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Configuration {
    pub file: String,
    pub keys: Vec<String>,
    pub dest: String,
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
    println!("Filetype: {:?}", &conf.file.split(".").last());
    match &conf.file.split(".").last() {
        Some(ending) => Some(Configuration {
            file: conf.file,
            keys: conf.keys,
            dest: conf.dest,
            filetype: ending.to_string(),
        }),
        None => None,
    }
}
