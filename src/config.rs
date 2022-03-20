use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Config {
    path: String,
    modes: Vec<String>,
}

fn get_config() -> Config {
    todo!();
}
