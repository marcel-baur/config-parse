mod config;
mod yaml_parser;

fn main() {
    config::get_config();
    yaml_parser::parse_yaml("./test.yaml");
}
