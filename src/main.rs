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
    tracing_subscriber::fmt::init();
    let args = Configuration::parse();
    tracing::info!("{:?}", args);
    match args.cli {
        false => {
            tracing::info!("From Config File");
            if let Some(conf) = config::get_config() {
                run(conf);
            }
        },
        true => {
            tracing::info!("From CLI Args");
            run(args);
        },
    };
}

fn run(args: Configuration) {
    let arc = Arc::new(args);
    let arc_lint = arc.clone();
    let _lint_handle = thread::spawn(move || {
        linter::lint(arc_lint.as_ref());
    });
    let _parse_handle = thread::spawn(move || {
        parse(arc);
    });
    _lint_handle.join().unwrap();
    _parse_handle.join().unwrap();
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
