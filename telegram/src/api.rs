use anyhow::{Error, Result};
use reqwest::{self};
use serde_json::{self, Value};

pub struct TGUrl<'a> {
    token: &'a String,
}

pub type TGUrlQuery<'a> = Vec<(&'a str, &'a str)>;

impl<'a> TGUrl<'a> {
    pub fn new(token: &'a String) -> Self {
        Self { token }
    }

    pub fn query(base: String, query: TGUrlQuery) -> String {
        let query = querystring::stringify(query);
        format!("{}?{}", base, query)
    }

    pub fn base(&self, method: &str) -> String {
        format!("https://api.telegram.org/bot{}/{}", self.token, method)
    }

    pub fn updates(&self, query: TGUrlQuery) -> String {
        Self::query(self.base("getUpdates"), query)
    }
}

pub struct TGApi<'a> {
    last_update: u64,
    admin_chat_id: &'a String,
    url: TGUrl<'a>,
}

impl<'a> TGApi<'a> {
    pub fn new(token: &'a String, admin_chat_id: &'a String) -> Self {
        Self {
            last_update: 0,
            admin_chat_id,
            url: TGUrl::new(token),
        }
    }

    pub fn get_updates(&mut self) -> Result<()> {
        let offset = self.last_update + 1;
        let offset = offset.to_string();
        let offset = offset.as_str();

        let url = self.url.updates(vec![("offset", offset)]);

        let txt = reqwest::blocking::get(url)?.text()?;
        let txt = txt.as_str();

        let json: Value = serde_json::from_str(txt)?;

        // println!("{:?}", json);

        let ok = json
            .get("ok")
            .ok_or(Error::msg("Could not get OK"))?
            .as_bool()
            .ok_or(Error::msg("Could not read OK as boolean"))?;

        if !ok {
            return Err(Error::msg("Result not ok"));
        }

        let results = json
            .get("result")
            .ok_or(Error::msg("Could not get results"))?
            .as_array()
            .ok_or(Error::msg("Could not read results as array"))?;

        for message in results {
            let update = message.as_object()
                .ok_or(Error::msg("Could not read update as map"))?;

            let last_update = update.get("update_id")
                .ok_or(Error::msg("Could not get update_id"))?
                .as_u64()
                .ok_or(Error::msg("Could not read update_id as u64"))?;

            let message = update.get("message")
                .ok_or(Error::msg("Could not read message of upset"))?
                .as_object()
                .ok_or(Error::msg("Could not read message as map"))?;

            let chat_id = message.get("chat").ok_or(Error::msg("X"))?
                .as_object().ok_or(Error::msg("X"))?
                .get("id").ok_or(Error::msg("X"))?
                .as_u64().ok_or(Error::msg("X"))?
                .to_string();

            if &chat_id != self.admin_chat_id {
                continue;
            }

            self.last_update = last_update;
            println!("{:?}", message);
        }

        Ok(())
    }
}
