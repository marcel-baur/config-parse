mod config;
mod yaml_parser;

fn main() {
    yaml_parser::parse_yaml("./test.yaml");
}
