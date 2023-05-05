extern crate yaml_rust;
use std::collections::HashMap;
use std::fs;
use tracing::{info, error, warn};
use yaml_rust::yaml::{Array, Hash};
use yaml_rust::{Yaml, YamlLoader};

use crate::config::Configuration;
use crate::model::Record;

/// The Path to the YAML Node should be provided in the Config file. It should
/// be an *exact* path, given in dot notation, e.g. `root.child.childtwo.node`.
/// Multiple Nodes can be queried.
/// Parse the Yaml file in the given `path`
pub fn parse_yaml(conf: &Configuration) -> Vec<Vec<Record>> {
    let mut records = Vec::new();
    for file in &conf.files {
        info!("Parsing file {}", &file);
        let doc: &Yaml = &load_yaml_file(file);
        let mut entries: Vec<Record> = vec![];
        for key in &conf.keys {
            let split: Vec<&str> = key.split('.').collect();
            match handle_split(split, doc) {
                Ok(yaml) => {
                    entries.push(handle_result(key.to_string(), yaml));
                }
                Err(msg) => {
                    error!("Cannot parse {file}: {msg}");
                }
            };
        }
        records.push(entries);
    }
    records
}

pub fn lint_yaml(conf: &Configuration) -> HashMap<String, Vec<String>> {
    let mut records = HashMap::new();
    for file in &conf.files {
        records.insert(file.clone(), Vec::new());
        let fname = file.clone();
        let doc: Yaml = load_yaml_file(file);
        let mut ve = Vec::<String>::new();
        lint_yaml_tree(doc, &mut ve, None);
        records.insert(fname, ve);
    }
    records
}

fn lint_yaml_tree(yaml: Yaml, list: &mut Vec<String>, cur_key: Option<String>) {
    match yaml {
        Yaml::Array(a) => {
            for yam in a {
                lint_yaml_tree(yam, list, cur_key.clone());
            }
        }
        Yaml::Hash(h) => {
            h.into_iter().for_each(|k| {
                let key = k.0;
                let val = k.1;
                let ck = match &cur_key {
                    Some(k) => {
                        let ch = k;
                        let ke = ch.to_owned() + ".";
                        String::try_into(ke).unwrap()
                    }
                    None => "".to_string(),
                };
                let cl = match key {
                    Yaml::Real(s) => ck + &s,
                    Yaml::String(s) => ck + &s,
                    Yaml::Integer(i) => ck + &i.to_string(),
                    Yaml::Boolean(b) => ck + &b.to_string(),
                    _ => {
                        warn!("Other val!");
                        "".to_string()
                    }
                };
                lint_yaml_tree(val, list, Some(cl));
            });
        }
        _ => {
            if let Some(val) = cur_key {
                list.push(val);
            };
        }
    };
    list.dedup();
}

fn handle_result(key: String, value: String) -> Record {
    let data: Record = Record { key, value };
    data
}

fn handle_split(split: Vec<&str>, doc: &Yaml) -> Result<String, &'static str> {
    let mut curr_yaml = doc.clone();
    for id in split {
        curr_yaml = curr_yaml[id].clone();
        match curr_yaml {
            Yaml::BadValue => {
                let err_msg: String = format!("Bad Value at: {}", id);
                error!("{}", err_msg);
                return Err("Bad Value!");
            }
            _ => {
                continue;
            }
        }
    }
    match handle_yaml(curr_yaml) {
        Some(s) => Ok(s),
        None => Err("Ambiguous yaml!"),
    }
}

/// Handle every possible value of the yaml and return it as a `Option<String>`
fn handle_yaml(yaml: Yaml) -> Option<String> {
    match yaml {
        Yaml::Real(s) => Some(s),
        Yaml::String(s) => Some(s),
        Yaml::Integer(i) => Some(i.to_string()),
        Yaml::Boolean(b) => Some(b.to_string()),
        Yaml::Array(a) => handle_yaml_array(a),
        Yaml::Hash(h) => handle_yaml_hash(h),
        Yaml::Alias(a) => Some(a.to_string()),
        _ => None,
    }
}

/// Handle a `Hash` representation of a `Yaml`. Return the path to the leaf if
/// it is a single path down to the leaf, an error code if there are multiple
/// possibilities
fn handle_yaml_hash(yaml: Hash) -> Option<String> {
    if yaml.len() > 1 {
        return Some(
            "The yaml path you provided is ambiguous! Please specify
            the exact key!"
                .to_string(),
        );
    }
    // The yaml hash only contains one key. We can thus recursively check the
    // next depth of the hash.
    let key = yaml.keys().next().unwrap();
    let rec_yaml: &Yaml = &yaml[key];
    handle_yaml(rec_yaml.clone())
}

fn handle_yaml_array(yaml: Array) -> Option<String> {
    if yaml.len() > 1 {
        return Some(
            "The yaml path you provided is ambiguous! Please specify the exact key!".to_string(),
        );
    }
    handle_yaml(yaml[0].clone())
}

fn load_yaml_file(path: &str) -> Yaml {
    let contents = fs::read_to_string(path).expect("Failed to read file!");
    let docs = YamlLoader::load_from_str(&contents).unwrap();
    docs[0].clone()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn yaml() {
        let doc = YamlLoader::load_from_str(
            &fs::read_to_string("test.yaml".to_string()).unwrap(),
        )
        .unwrap();
        let mut list = Vec::<String>::new();
        lint_yaml_tree(doc[0].clone(), &mut list, None);
        info!("{:?}", list);
    }
}
