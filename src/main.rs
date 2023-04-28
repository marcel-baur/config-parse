use std::{sync::Arc, thread};

use crate::config::Configuration;
use linter::{Filetype, fetch_file_types};
use clap::Parser;

mod config;
mod linter;
mod model;
mod properties_parser;
mod writer;
mod yaml_parser;

fn main() {
    log4rs::init_file("log4rs.yml", Default::default()).unwrap();
    let args = Configuration::parse();
    println!("{:?}", args);
    match args.cli {
        false => {
            log::info!("From Config File");
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
        },
        true => {
            log::info!("From CLI Args");
            let arc = Arc::new(args.clone());
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
        },
    };
}

fn parse(arc_parse: Arc<Configuration>) {
    let configuration = arc_parse.as_ref().clone();
    let filetype = fetch_file_types(&configuration);
    match filetype {
        Filetype::Properties => {
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
        Filetype::Yaml => {
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
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_load_config() {
        let configuration = config::get_config().unwrap();
        linter::lint(&configuration);
    }
}
