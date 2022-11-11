use serde::{self, Deserialize, Serialize};

#[derive(Deserialize, Debug)]
pub struct JsonApiIndex {
    pub ok: bool,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum JsonMediaType {
    PHOTO,
    VIDEO,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct JsonMedia {
    pub id: String,

    #[serde(rename = "type")]
    pub media_type: JsonMediaType,

    pub source: String,
    pub thumb: Option<String>,
    pub lq: Option<String>,
    pub hq: Option<String>,

    pub width: u64,
    pub height: u64,
    pub length: u64,
}
