use std::{
    sync::mpsc::{self, Sender},
    thread,
};
use anyhow::{Error, Result};

mod api;
use api::{TGApi, TGMessage, TGFile};
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
                    let _ = api.send(message);
                });
            }
        })
        .unwrap()
    });

    let rx_thread = thread::spawn(move || {
        println!("TG rx thread started.");
        let mut api = TGApi::new_from_env();

        let _ = tx_message.send("Bot ready. Share stories about your day! 🤖".to_string());
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

    let _ = tx_thread.join().unwrap();
    let _ = rx_thread.join().unwrap();
}

fn handle_message(message: TGMessage, state: TGState, tx: Sender<String>) -> TGState {
    println!("GOT MESSAGE: {:?}", message);

    match state {
        TGState::Start => match message.command {
            BotCommand::MakePostStream => {
                let _ = tx.send(format!("Starting post stream! 🎈"));
                TGState::MakePostStream
            },
            BotCommand::MakePost(post_text) => {
                let _ = handle_make_post_on_new_thread(post_text, message.file_id, tx.clone());
                TGState::Start
            },
            _ => {
                let _ = tx.send(format!("I don't know what to do with `{:?}`. 😥", message.command));
                TGState::Start
            }
        },
        TGState::MakePostStream => match message.command {
            BotCommand::Done => {
                let _ = tx.send("Post stream ended. 🤖".to_string());
                TGState::Start
            },
            _ => {
                let post_text = parse_make_post(&message.text);
                let _ = handle_make_post_on_new_thread(post_text, message.file_id, tx.clone());
                TGState::MakePostStream
            }
        }
    }
}

fn handle_make_post_on_new_thread(post_text: PostText, file_id: Option<String>, tx: Sender<String>) {
    thread::spawn(move || {
        let _ = handle_make_post(post_text, file_id, tx);
    });
}

fn handle_make_post(post_text: PostText, file_id: Option<String>, tx: Sender<String>) -> Result<()>{
    let response = format!("Making post `{:?} with files {:?}`", post_text, file_id);

    if let Some(file_id) = file_id {
        let _ = tx.send(format!("Downloading..."));
        let api = TGApi::new_from_env();
        let file = api.get_file_url(&file_id)?;
        let _ = tx.send(format!("Loaded {} file.", file.ext));
    }
    
    let _ = tx.send(response);

    Ok(())
}