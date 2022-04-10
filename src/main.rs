mod config;
mod model;
mod properties_parser;
mod writer;
mod yaml_parser;

fn main() {
    if let Some(configuration) = config::get_config() {
        match configuration.filetype.as_str() {
            "properties" => {
                properties_parser::parse(configuration);
            }
            "yaml" => {
                yaml_parser::parse_yaml(configuration);
            }
            _ => {}
        }
    }
}
