extern crate yaml_rust;
use std::fs;
use yaml_rust::yaml::{Array, Hash};
use yaml_rust::{Yaml, YamlLoader};

/// The Path to the YAML Node should be provided in the Config file. It should
/// be an *exact* path, given in dot notation, e.g. `root.child.childtwo.node`.
/// Multiple Nodes can be queried.
/// Parse the Yaml file in the given `path`
pub fn parse_yaml(path: &str) {
    println!("Parsing file {}", path);
    let doc: &Yaml = &load_yaml_file(path);
    if let Some(stringified) = handle_yaml(doc["one"].clone()) {
        println!("Some value: {}", stringified);
    }
    println!("{:?}", doc["one"]);
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

/// Handle a `Hash` representation of a `Yaml`. Return the path to the leaf if it is a
/// single path down to the leaf, an error code if there are multiple possibilities
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
