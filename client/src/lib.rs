mod json;
pub mod url;
use std::env;

pub use url::*;

use anyhow::{Error, Result};

use crate::json::ResApiIndex;

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

    pub fn test_connection() -> Result<ResApiIndex> {
        let response = Self::req(ReqMeth::GET, "".to_string()).send()?;
        if response.status() != 200 {
            return Err(Error::msg("Could not connect to API."));
        }
        let json = response.json::<ResApiIndex>()?;
        Ok(json)
    }
}
