use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct Record {
    pub key: String,
    pub value: String,
}
