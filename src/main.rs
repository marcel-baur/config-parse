use config_parse::hello;
mod config;
mod model;
mod properties_parser;
mod writer;
mod yaml_parser;

fn main() {
    hello();
    if let Some(configuration) = config::get_config() {
        match configuration.filetype.as_str() {
            "properties" => {
                properties_parser::write_keys(configuration);
            }
            "yaml" => {
                yaml_parser::write_yaml_keys(configuration);
            }
            _ => {}
        }
    }
}
