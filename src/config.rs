use config::Config;
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
struct Configuration {
    keys: Vec<String>
}

pub fn get_config() {
    let settings = Config::builder()
        .add_source(config::File::with_name("config_parser"))
        .add_source(config::Environment::with_prefix("APP"))
        .build()
        .unwrap();

    println!(
        "Config: {:?}",
        settings
        .try_deserialize::<Configuration>()
        .unwrap()
        );
}
