mod config;
mod model;
mod properties_parser;
mod writer;
mod yaml_parser;

fn main() {
    if let Some(configuration) = config::get_config() {
        match configuration.filetype.as_str() {
            "properties" => {
                let records = properties_parser::parse_new(&configuration);
                for (idx,file) in configuration.files.into_iter().enumerate() {
                    match writer::write(&records[idx], file.to_string()) {
                        Ok(()) => {}
                        Err(_e) => {}
                    }
                }
            }
            "yaml" => {
                let records = yaml_parser::parse_yaml(&configuration);
                for (idx,file) in configuration.files.into_iter().enumerate() {
                    match writer::write(&records[idx], file.to_string()) {
                        Ok(()) => {}
                        Err(_e) => {}
                    }
                }
            }
            _ => {}
        }
    }
}
