use anyhow::{Error, Result};
use std::env;

pub mod json;
pub use crate::json::*;

pub mod url;
pub use crate::url::*;

pub struct APIClient {}

pub enum ReqMeth {
    GET,
    POST,
}

impl APIClient {
    pub fn token() -> String {
        env::var("API_TOKEN").expect("API_TOKEN not set in the environment")
    }

    pub fn req(meth: ReqMeth, url: String) -> reqwest::blocking::RequestBuilder {
        let url = get_api_url(&url);
        let builder = reqwest::blocking::Client::new();
        let builder = match meth {
            ReqMeth::GET => builder.get(url),
            ReqMeth::POST => builder.post(url),
        };
        let builder = builder.header(reqwest::header::AUTHORIZATION, Self::token());
        builder
    }

    pub fn test_connection() -> Result<JsonApiIndex> {
        let response = Self::req(ReqMeth::GET, "".to_string()).send()?;
        if response.status() != 200 {
            return Err(Error::msg("Could not connect to API."));
        }
        let json = response.json::<JsonApiIndex>()?;
        Ok(json)
    }

    pub fn upsert_media(media: JsonMedia) -> Result<JsonMedia> {
        let response = Self::req(ReqMeth::POST, "media".to_string())
            .json(&media)
            .send()?;
        if response.status() != 200 {
            return Err(Error::msg(format!(
                "Could not upsert_media. {:?}",
                response.text()
            )));
        }

        let json = response.json::<JsonMedia>()?;
        Ok(json)
    }
}
