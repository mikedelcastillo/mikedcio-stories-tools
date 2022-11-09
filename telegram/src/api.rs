use std::env;

use anyhow::{Error, Result};
use reqwest::{self};
use serde_json::{self, Map, Value};
use urlencoding::encode;
use utils::{parse_message, BotCommand};

pub struct TGUrl {
    token: String,
}

pub type TGUrlQuery<'a> = Vec<(&'a str, &'a str)>;

impl TGUrl {
    pub fn new(token: String) -> Self {
        Self { token }
    }

    pub fn query<'b>(base: String, query: TGUrlQuery<'b>) -> String {
        let mut query_string = String::new();

        for (key, val) in query {
            let encoded = encode(val).into_owned();
            let part = format!("{}={}&", key, encoded);
            query_string.push_str(part.as_str())
        }

        format!("{}?{}", base, query_string)
    }

    pub fn base(&self, method: &str) -> String {
        format!("https://api.telegram.org/bot{}/{}", self.token, method)
    }

    pub fn file_path(&self, path: &String) -> String {
        format!("https://api.telegram.org/file/bot{}/{}", self.token, path)
    }

    pub fn updates(&self, query: TGUrlQuery) -> String {
        Self::query(self.base("getUpdates"), query)
    }

    pub fn send(&self, query: TGUrlQuery) -> String {
        Self::query(self.base("sendMessage"), query)
    }

    pub fn file(&self, query: TGUrlQuery) -> String {
        // file_id=string
        Self::query(self.base("getFile"), query)
    }
}

#[derive(Debug)]
pub struct TGMessage {
    pub update_id: u64,
    pub text: String,
    pub command: BotCommand,
    pub group_id: Option<String>,
    pub file_id: Option<String>,
}

#[derive(Debug)]
pub struct TGFile {
    pub url: String,
    pub ext: String,
}

pub struct TGApi {
    last_update: u64,
    admin_chat_id: String,
    url: TGUrl,
}

impl TGApi {
    pub fn new(token: String, admin_chat_id: String) -> Self {
        Self {
            last_update: 0,
            admin_chat_id,
            url: TGUrl::new(token),
        }
    }

    pub fn new_from_env() -> Self {
        let token = env::var("TELEGRAM_BOT_ACCESS_TOKEN")
            .expect("TELEGRAM_BOT_ACCESS_TOKEN not set in environment");
        let admin_chat_id = env::var("TELEGRAM_ADMIN_CHAT_ID")
            .expect("TELEGRAM_ADMIN_CHAT_ID not set in environment");

        Self::new(token, admin_chat_id)
    }

    pub fn send(&self, message: String) -> Result<()> {
        println!("SENDING: {}", message);
        let url = self.url.send(vec![
            ("chat_id", self.admin_chat_id.as_str()),
            ("text", message.as_str()),
        ]);

        match reqwest::blocking::get(url) {
            Ok(_) => Ok(()),
            _ => Err(Error::msg("Could not send message.")),
        }
    }

    pub fn _send_multiple(&self, messages: Vec<String>) -> Result<()> {
        match crossbeam::thread::scope(|s| {
            for message in messages {
                s.spawn(|_| {
                    let _ = self.send(message);
                });
            }
            Ok(())
        }) {
            Ok(v) => v,
            _ => Err(Error::msg("Could not send messages.")),
        }
    }

    fn get_message_file_id(obj: &Map<String, Value>) -> Result<String> {
        Ok(obj
            .get("file_id")
            .ok_or(Error::msg("Could not get file_id from document"))?
            .as_str()
            .ok_or(Error::msg("Could not read file_id as string"))?
            .to_string())
    }

    fn get_message_file_size(obj: &Map<String, Value>) -> Result<u64> {
        Ok(obj
            .get("file_size")
            .ok_or(Error::msg("Could not get file_size from document"))?
            .as_u64()
            .ok_or(Error::msg("Could not read file_size as number"))?)
    }

    fn get_message_media(message: &Map<String, Value>) -> Result<Option<String>> {
        if let Some(document) = message.get("document") {
            let document = document
                .as_object()
                .ok_or(Error::msg("Could not read document as object"))?;

            return Ok(Some(Self::get_message_file_id(document)?));
        };
        if let Some(video) = message.get("video") {
            let video = video
                .as_object()
                .ok_or(Error::msg("Could not read video as object"))?;

            return Ok(Some(Self::get_message_file_id(video)?));
        };
        if let Some(photos) = message.get("photo") {
            let photos = photos
                .as_array()
                .ok_or(Error::msg("Could not read photo as array"))?;

            let mut photo_objs = vec![];

            for photo in photos {
                match photo.as_object() {
                    Some(photo) => photo_objs.push(photo),
                    _ => (),
                };
            }

            if photos.len() >= 1 {
                let mut largest_photo_size: u64 = 0;
                let mut largest_photo = photo_objs[0];

                for photo in &photo_objs[1..] {
                    let file_size = Self::get_message_file_size(photo)?;
                    if file_size > largest_photo_size {
                        largest_photo = *photo;
                        largest_photo_size = file_size;
                    }
                }

                return Ok(Some(Self::get_message_file_id(largest_photo)?));
            }
        }

        Ok(None)
    }

    pub fn get_updates(&mut self) -> Result<Vec<TGMessage>> {
        let mut temp_last_update = self.last_update;
        let offset = self.last_update + 1;
        let offset = offset.to_string();
        let offset = offset.as_str();

        let url = self.url.updates(vec![("offset", offset)]);

        let response_text = reqwest::blocking::get(url)?.text()?;
        let response_text = response_text.as_str();

        let json: Value = serde_json::from_str(response_text)?;

        let mut update_messages: Vec<TGMessage> = vec![];

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
            let update = message
                .as_object()
                .ok_or(Error::msg("Could not read update as map"))?;

            let update_id = update
                .get("update_id")
                .ok_or(Error::msg("Could not get update_id"))?
                .as_u64()
                .ok_or(Error::msg("Could not read update_id as u64"))?;

            let message = update
                .get("message")
                .ok_or(Error::msg("Could not read message of upset"))?
                .as_object()
                .ok_or(Error::msg("Could not read message as map"))?;

            let chat_id = message
                .get("chat")
                .ok_or(Error::msg("Could not get chat of message"))?
                .as_object()
                .ok_or(Error::msg("Could not read chat as object"))?
                .get("id")
                .ok_or(Error::msg("Could not get id of chat"))?
                .as_u64()
                .ok_or(Error::msg("Could not read if as u64"))?
                .to_string();

            if chat_id != self.admin_chat_id {
                continue;
            };

            let group_id = match message.get("media_group_id") {
                Some(media_group_id) => Some(
                    media_group_id
                        .as_str()
                        .ok_or(Error::msg("Could not read media_group_id as string"))?
                        .to_string(),
                ),
                _ => None,
            };

            let file_id = Self::get_message_media(message)?;

            let message_text = match message.get("text") {
                Some(text) => text
                    .as_str()
                    .ok_or(Error::msg("Could not read text as string"))?,
                None => "",
            };

            let message_caption = match message.get("caption") {
                Some(caption) => caption
                    .as_str()
                    .ok_or(Error::msg("Could not read caption as string"))?,
                None => "",
            };

            let combined_text = format!("{} {}", message_text, message_caption);
            let combined_text = combined_text.trim();

            // If the text is empty, get the caption of previous group members
            let combined_text = {
                let mut temp_combined_text = combined_text.to_string();

                if combined_text.len() == 0 {
                    if let Some(a_id) = &group_id {
                        for up_msg in update_messages.iter().rev() {
                            if let Some(b_id) = &up_msg.group_id {
                                if a_id == b_id {
                                    if up_msg.text.len() > 0 {
                                        temp_combined_text = up_msg.text.to_owned();
                                        break;
                                    }
                                }
                            }
                        }
                    }
                };

                temp_combined_text
            };

            let command = parse_message(&combined_text);

            update_messages.push(TGMessage {
                update_id,
                text: combined_text,
                command,
                group_id,
                file_id,
            });

            temp_last_update = update_id;
        }

        self.last_update = temp_last_update;

        Ok(update_messages)
    }

    pub fn get_file_url(&self, file_id: &String) -> Result<TGFile> {
        let url = self.url.file(vec![("file_id", file_id.as_str())]);

        let response_text = reqwest::blocking::get(url)?.text()?;
        let response_text = response_text.as_str();
        let json: Value = serde_json::from_str(response_text)?;

        let file_path = json
            .as_object()
            .ok_or(Error::msg("Could not read json as object"))?
            .get("result")
            .ok_or(Error::msg("Could not get result from object"))?
            .as_object()
            .ok_or(Error::msg("Could not read result as object"))?
            .get("file_path")
            .ok_or(Error::msg("Could not get file_path from result"))?
            .as_str()
            .ok_or(Error::msg("Could not read file_path as string"))?
            .to_string();

        let ext = std::path::Path::new(file_path.as_str());
        let ext = ext
            .extension()
            .unwrap_or_default()
            .to_str()
            .unwrap_or_default()
            .to_lowercase()
            .to_string();

        println!("THE EXT IS {}", ext);

        let file_url = self.url.file_path(&file_path);

        Ok(TGFile { url: file_url, ext })
    }
}
