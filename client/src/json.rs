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

#[derive(Serialize, Deserialize, Debug)]
pub enum JsonStoryType {
    MEDIA,
    LINK,
    GROUP,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct JsonStory {
    pub id: String,
    pub title: String,
    pub caption: String,

    #[serde(rename = "contentType")]
    pub content_type: JsonStoryType,

    #[serde(rename = "contentLink")]
    pub content_link: Option<String>,

    #[serde(rename = "contentMediaId")]
    pub content_media_id: Option<String>,

    #[serde(rename = "contentGroupId")]
    pub content_group_id: Option<String>,
}
