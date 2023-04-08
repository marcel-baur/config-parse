use std::collections::HashMap;

use propparse::parse_file;

use crate::{config::Configuration, yaml_parser::lint_yaml};

use colored::Colorize;

pub enum Filetype {
    Yaml,
    Properties,
}

pub fn fetch_file_types(config: &Configuration) -> Filetype {
    let files = config.files.clone();
    let endings: Vec<String> = files
        .into_iter()
        // .map(|f| f.to_string())
        .map(|f| {
            let split: Vec<&str> = f.split(".").collect();
            let last = split.last().unwrap().to_string();
            last
        })
        .collect();
    if endings.len() == 0 {
        panic!("No files!");
    }
    let first = endings[0].clone();
    if endings.into_iter().all(|e| e == first) {
        return match first.as_str() {
            "properties" => Filetype::Properties,
            "yaml" => Filetype::Yaml,
            _ => panic!("Filetype {first} not supported"),
        };
    }
    panic!("Inconsisten filetypes!");
}

pub fn lint_files(
    config: &Configuration,
    ftype: Filetype,
) -> HashMap<String, Vec<String>> {
    match ftype {
        Filetype::Properties => {
            let mut res = HashMap::new();
            for file in config.clone().files {
                let parsed = match parse_file(file.as_str()) {
                    Ok(r) => r,
                    Err(e) => {
                        panic!("Error: {:?}", e);
                    }
                }
                .into_iter()
                .map(|r| r.0.join("."))
                .collect();
                res.insert(file.clone(), parsed);
            }
            res
        }
        Filetype::Yaml => lint_yaml(config),
    }
}

pub fn lint(configuration: &Configuration) {
    let ftype = fetch_file_types(&configuration);
    let mut key_map = HashMap::<String, Vec<String>>::new();

    let linted = lint_files(configuration, ftype);

    for file in linted {
        let keys = file.1;
        for key in keys {
            match key_map.get_mut(&key) {
                Some(res) => {
                    res.push(file.0.clone());
                }
                None => {
                    key_map.insert(key, vec![file.0.clone()]);
                }
            }
        }
    }
    let outliers: Vec<(String, Vec<String>)> = key_map
        .into_iter()
        .filter(|e| e.1.len() < configuration.files.len())
        .collect();
    println!(
        "The following properties only appear in the listed files: {:?}",
        outliers
    );
    for outlier in outliers {
        let variable = outlier.0;
        println!(
            "{}: {}{}",
            "Outiler".bold(),
            variable.bold().yellow(),
            ". This parameter dows not appear in the following files:"
        );
        let not_in: Vec<String> = configuration
            .files
            .clone()
            .into_iter()
            .filter(|f| !outlier.1.contains(f))
            .collect();
        for file in not_in {
            println!("    {}", file.yellow());
        }
    }
}
