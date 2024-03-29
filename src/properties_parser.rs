use crate::config::Configuration;
use crate::model::Record;

pub fn parse_new(configuration: &Configuration) -> Vec<Vec<Record>> {
    let mut records = Vec::new();
    for file in &configuration.files {
        let parsed = propparse::parse_file(file);
        let result = match parsed {
            Ok(p) => {
                let iter = p.into_iter();
                let val = iter
                    .map(|r| {
                        let value = match r.1 {
                            propparse::types::Value::Null => "".to_string(),
                            propparse::types::Value::Integer(i) => {
                                i.to_string()
                            }
                            propparse::types::Value::String(s) => s,
                        };

                        Record {
                            key: r.0.join("."),
                            value,
                        }
                    })
                    .collect::<Vec<Record>>();
                val
            }
            Err(e) => {
                tracing::error!("{:?}", e);
                Vec::<Record>::new()
            }
        };
        records.push(result);
    }
    records
}
