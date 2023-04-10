use std::{sync::Arc, thread};

use crate::config::Configuration;
use linter::Filetype;

mod config;
mod linter;
mod model;
mod properties_parser;
mod writer;
mod yaml_parser;

fn main() {
    log4rs::init_file("log4rs.yml", Default::default()).unwrap();
    if let Some(conf) = config::get_config() {
        let arc = Arc::new(conf.clone());
        let arc_lint = arc.clone();
        let arc_parse = arc.clone();
        let lint_handle = thread::spawn(move || {
            linter::lint(arc_lint.as_ref());
        });
        let parse_handle = thread::spawn(move || {
            parse(arc_parse);
        });
        lint_handle.join().unwrap();
        parse_handle.join().unwrap();
    }
}

fn parse(arc_parse: Arc<Configuration>) {
    let configuration = arc_parse.as_ref().clone();
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

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_lint() {
        let configuration = config::get_test_config().unwrap();
        linter::lint(&configuration);
    }
}
