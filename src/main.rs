mod yaml_parser;
mod config;

fn main() {
    yaml_parser::parse_yaml("./test.yaml");
}
