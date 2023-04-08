use linter::Filetype;

mod config;
mod linter;
mod model;
mod properties_parser;
mod writer;
mod yaml_parser;

fn main() {
    if let Some(configuration) = config::get_config() {
        linter::lint(&configuration);
        match configuration.filetype.as_str() {
            "properties" => {
                let records = properties_parser::parse_new(&configuration);
                for (idx, file) in
                    configuration.clone().files.into_iter().enumerate()
                {
                    match writer::write(&records[idx], file.to_string()) {
                        Ok(()) => {}
                        Err(_e) => {}
                    }
                }
                linter::lint_files(&configuration, Filetype::Properties);
            }
            "yaml" => {
                let records = yaml_parser::parse_yaml(&configuration);
                for (idx, file) in
                    configuration.clone().files.into_iter().enumerate()
                {
                    match writer::write(&records[idx], file.to_string()) {
                        Ok(()) => {}
                        Err(_e) => {}
                    }
                }
                linter::lint_files(&configuration, Filetype::Yaml);
            }
            _ => {}
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_lint() {
        let configuration = config::get_test_config().unwrap();
        linter::lint(&configuration);
    }
}
