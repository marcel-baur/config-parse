mod config;
mod model;
mod writer;
mod yaml_parser;

fn main() {
    let configuration = config::get_config();
    yaml_parser::parse_yaml(configuration);
}
