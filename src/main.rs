mod config;
mod yaml_parser;
mod writer;
mod model;

fn main() {
    let configuration = config::get_config();
    yaml_parser::parse_yaml("./test.yaml", configuration);
}
