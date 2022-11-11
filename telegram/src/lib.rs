use anyhow::{Error, Result};
use client::{get_bucket_url, APIClient, JsonMedia, JsonMediaType};
use std::sync::mpsc::{self, Sender};
use std::thread;

use files::{download_from_url, ext_to_type, get_file_ext, ActiveRemote, MediaType, Remote};

mod api;

use api::{TGApi, TGMessage};
use utils::{parse_make_post, BotCommand, PostText};

#[derive(Debug)]
pub enum TGState {
    Start,
    MakePostStream,
}

pub fn run_telegram_bot() {
    let (tx_message, rx_message) = mpsc::channel::<String>();

    let tx_thread = thread::spawn(move || {
        println!("TG tx thread started.");
        let api = TGApi::new_from_env();
        crossbeam::thread::scope(|s| {
            for message in rx_message {
                s.spawn(|_| {
                    api.send(message);
                });
            }
        })
        .unwrap()
    });

    let rx_thread = thread::spawn(move || {
        println!("TG rx thread started.");
        let mut api = TGApi::new_from_env();

        tx_message.send("Bot ready. Share stories about your day! 🤖".to_string()).ok();
        let mut state = TGState::Start;

        loop {
            match api.get_updates() {
                Err(err) => println!("{:?}", err),
                Ok(messages) => {
                    for message in messages {
                        state = handle_message(message, state, tx_message.clone());
                    }
                }
            }
        }
    });

    tx_thread.join().expect("Could not join tx_thread");
    rx_thread.join().expect("Could not join rx_thread");
}

fn handle_message(message: TGMessage, state: TGState, tx: Sender<String>) -> TGState {
    println!("GOT MESSAGE: {:?}", message);

    match state {
        TGState::Start => match message.command {
            BotCommand::MakePostStream => {
                tx.send(format!("Starting post stream! 🎈")).ok();
                TGState::MakePostStream
            }
            BotCommand::MakePost(post_text) => {
                handle_make_post_on_new_thread(post_text, message.file_id, tx.clone());
                TGState::Start
            }
            _ => {
                tx.send(format!(
                    "I don't know what to do with `{:?}`. 😥",
                    message.command
                )).ok();
                TGState::Start
            }
        },
        TGState::MakePostStream => match message.command {
            BotCommand::Done => {
                tx.send("Post stream ended. 🤖".to_string()).ok();
                TGState::Start
            }
            _ => {
                let post_text = parse_make_post(&message.text);
                handle_make_post_on_new_thread(post_text, message.file_id, tx.clone());
                TGState::MakePostStream
            }
        },
    }
}

#[allow(unused)]
fn handle_make_post_on_new_thread(
    post_text: PostText,
    file_id: Option<String>,
    tx: Sender<String>,
) {
    thread::spawn(move || {
        match handle_make_post(post_text, file_id, tx.clone()) {
            Ok(_) => {}
            Err(err) => {
                tx.send(format!("MakePostError: {:?}", err)).ok();
            }
        };
    });
}

fn handle_make_post(
    post_text: PostText,
    file_id: Option<String>,
    tx: Sender<String>,
) -> Result<()> {
    let response = format!("{:?}:{:?}`", &post_text, &file_id);

    if let Some(file_id) = file_id {
        let api = TGApi::new_from_env();
        let file_url = api.get_file_url(&file_id)?;

        let media_type = get_file_ext(&file_url)?;
        let media_type = ext_to_type(&media_type);

        if let MediaType::Unknown = &media_type {
            return Err(Error::msg("File type is unknown."));
        }

        let media_type = match &media_type {
            MediaType::Photo(_) => JsonMediaType::PHOTO,
            MediaType::Video(_) => JsonMediaType::VIDEO,
            MediaType::Unknown => unreachable!(),
        };

        tx.send(format!("Downloading...")).ok();

        let (file_id, file_path) = download_from_url(file_url)?;

        tx.send(format!("Uploading: {:?}", file_path)).ok();

        let file_name = ActiveRemote::upload(file_path)?;

        let thumb = match &media_type {
            JsonMediaType::PHOTO => Some(file_name.clone()),
            JsonMediaType::VIDEO => None,
        };

        let media_json = JsonMedia {
            id: file_id,
            media_type,
            source: file_name.clone(),
            thumb,
            lq: Some(file_name.clone()),
            hq: Some(file_name.clone()),
            width: 0,
            height: 0,
            length: 0,
        };

        APIClient::upsert_media(media_json)?;

        let url = get_bucket_url(&file_name);
        tx.send(format!("Resource: {}", url)).ok();
    } else {
        let sum = post_text.link.len();
        if sum == 0 {
            return Err(Error::msg("Post content is empty"));
        }
    }

    tx.send(response).ok();

    Ok(())
}
