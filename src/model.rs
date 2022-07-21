use serde::Serialize;

#[derive(Clone, Debug, PartialEq, PartialOrd, Serialize)]
pub struct Record {
    pub key: String,
    pub value: String,
}
