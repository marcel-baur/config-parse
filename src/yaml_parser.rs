extern crate yaml_rust;
use std::fs;
use serde::Serialize;
use yaml_rust::yaml::{Array, Hash};
use yaml_rust::{Yaml, YamlLoader};

use crate::model::Record;
use crate::writer::write;
use crate::config::Configuration;

/// The Path to the YAML Node should be provided in the Config file. It should
/// be an *exact* path, given in dot notation, e.g. `root.child.childtwo.node`.
/// Multiple Nodes can be queried.
/// Parse the Yaml file in the given `path`
pub fn parse_yaml(path: &str, conf: Configuration) {
    println!("Parsing file {}", path);
    let doc: &Yaml = &load_yaml_file(path);
    let mut entries: Vec<Record> = vec![];
    for key in conf.keys {
        let split: Vec<&str> = key.split(".").collect();
        match handle_split(split, &doc) {
            Ok(yaml) => {
                println!("{}", yaml);
                entries.push(handle_result(key, yaml));
            }
            Err(msg) => {
                println!("{}", msg);
            }
        };
    }
    match write(entries, conf.dest) {
        Ok(()) => {},
        Err(_e) => {},
    }
}

fn handle_result(key: String, value: String) -> Record {
    let data: Record = Record { key, value };
    data
}

fn handle_split(split: Vec<&str>, doc: &Yaml) -> Result<String, &'static str> {
    let mut curr_yaml = doc.clone();
    for id in split {
        // println!("{}", id);
        curr_yaml = curr_yaml[id].clone();
        match curr_yaml {
            Yaml::BadValue => {
                let err_msg: String =
                    format!("Bad Value at: {}", id).to_string();
                println!("{}", err_msg);
                return Err("Bad Value!");
            }
            _ => {
                continue;
            }
        }
    }
    println!("{:?}", curr_yaml);
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
    println!("Handling hash : {:?}", yaml);
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
    return handle_yaml(rec_yaml.clone());
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
    let doc: Yaml = docs[0].clone();
    return doc;
}
