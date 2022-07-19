use serde::{Deserialize, Serialize};
#[derive(Serialize, Deserialize, Debug)]
pub(crate) struct ComplicatedBody<'a> {
    pub(crate) secretapikey: &'a str,
    pub apikey: &'a str,
    pub name: &'a str,
    #[serde(rename = "type")]
    pub dtype: &'a str,
    pub content: &'a str,
    pub ttl: &'a str,
}

#[derive(Serialize, Deserialize, Debug)]
pub(crate) struct SimpleBody<'a> {
    pub secretapikey: &'a str,
    pub apikey: &'a str,
    pub content: &'a str,
    pub ttl: &'a str,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Keys {
    pub secretapikey: String,
    pub apikey: String,
}

/// Record contains the content of the record itself.
#[derive(Serialize, Deserialize, Debug)]
pub struct Record {
    pub name: String,
    #[serde(rename = "type")]
    pub dtype: String,
    pub content: String,
    pub ttl: String,
}
/// Record info contains all the information that is attatched to the record such as id
/// , your notes and record priority.
#[derive(Serialize, Deserialize, Debug)]
pub struct RecordInfo {
    #[serde(flatten)]
    pub record: Record,
    pub id: String,
    pub prio: Option<String>,
    pub notes: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Certificate {
    #[serde(rename = "intermediatecertificate")]
    pub intermediate_certificate: String,
    #[serde(rename = "certificatechain")]
    pub certificate_chain: String,
    #[serde(rename = "privatekey")]
    pub private_key: String,
    #[serde(rename = "publickey")]
    pub public_key: String,
}
