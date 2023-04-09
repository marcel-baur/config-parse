use std::collections::HashMap;

use propparse::parse_file;

use crate::{config::Configuration, yaml_parser::lint_yaml};

use colored::Colorize;

#[derive(Debug, PartialEq)]
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config;
    fn get_yaml_config() -> Configuration {
        Configuration {
            files: vec!["test/1.yaml".to_string(), "test/2.yaml".to_string()],
            keys: vec!["one.big.cascade".to_string(), "url".to_string()],
            filetype: "yaml".to_string(),
        }
    }

    fn get_prop_config() -> Configuration {
        Configuration {
            files: vec![
                "test/1.properties".to_string(),
                "test/2.properties".to_string(),
            ],
            keys: vec![
                "this.is.a".to_string(),
                "this.counts".to_string(),
                "this.is.mixed".to_string(),
            ],
            filetype: "properties".to_string(),
        }
    }

    #[test]
    fn test_lint_yaml() {
        let config = get_yaml_config();
        let result = lint_files(&config, Filetype::Yaml);
        let file_one_keys = vec![
            "security.auth.cidaas.client_id",
            "security.auth.cidaas.client_secret",
            "one.big.cascade",
            "url",
            "value",
        ];
        let file_two_keys = vec![
            "security.auth.cidaas.client_id",
            "security.auth.cidaas.client_secret",
            "security.auth.gcloud",
            "one.big.cascade",
            "url",
            "value",
        ];
        let f1: Vec<String> =
            file_one_keys.into_iter().map(|k| k.to_string()).collect();
        let f2: Vec<String> =
            file_two_keys.into_iter().map(|k| k.to_string()).collect();
        let mut expected = HashMap::new();
        expected.insert("test/1.yaml".to_string(), f1);
        expected.insert("test/2.yaml".to_string(), f2);
        assert_eq!(result, expected);
    }

    #[test]
    fn test_lint_properties() {
        let config = get_prop_config();
        let result = lint_files(&config, Filetype::Properties);
        let file_one_keys =
            vec!["this.is.a", "this.counts", "this.is.mixed", "url"];
        let file_two_keys = vec![
            "this.is.a",
            "this.counts",
            "this.is.mixed",
            "url",
            "only.two",
        ];
        let f1: Vec<String> =
            file_one_keys.into_iter().map(|k| k.to_string()).collect();
        let f2: Vec<String> =
            file_two_keys.into_iter().map(|k| k.to_string()).collect();
        let mut expected = HashMap::new();
        expected.insert("test/1.properties".to_string(), f1);
        expected.insert("test/2.properties".to_string(), f2);
        assert_eq!(result, expected);
    }

    #[test]
    fn fetches_yaml() {
        let config = get_yaml_config();
        let result = fetch_file_types(&config);
        assert_eq!(Filetype::Yaml, result);
    }

    #[test]
    fn fetches_prop() {
        let config = get_prop_config();
        let result = fetch_file_types(&config);
        assert_eq!(Filetype::Properties, result);
    }

    #[test]
    #[should_panic]
    fn fetches_err() {
        let config = Configuration {
            files: vec!["1.properties".to_string(), "2.yaml".to_string()],
            filetype: "yaml".to_string(),
            keys: Vec::new(),
        };
        let result = fetch_file_types(&config);
    }
}
