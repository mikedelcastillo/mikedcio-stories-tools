use reqwest::{self, Error};
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
    url: TGUrl<'a>,
}

impl<'a> TGApi<'a> {
    pub fn new(token: &'a String) -> Self {
        Self {
            url: TGUrl::new(token),
        }
    }

    pub fn get_updates(&mut self) -> Result<u32, Box<dyn std::error::Error>> {
        let url = self.url.updates(vec![]);

        let txt = reqwest::blocking::get(url)?.text()?;
        let txt = txt.as_str();
        println!("{}", txt);

        let json: Value = serde_json::from_str(txt)?;

        println!("{:?}", json);

        json.get("index")

        Ok(69)
    }
}
