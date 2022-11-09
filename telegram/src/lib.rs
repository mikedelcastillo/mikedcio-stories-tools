use std::{
    sync::mpsc::{self, Sender},
    thread,
};

mod api;
use api::{TGApi, TGMessage};
use utils::{BotMessage, parse_make_post};

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
    println!("{:?}", message);
    match message.parsed {
        Ok(parsed) => match state {
            TGState::Start => {
                match parsed {
                    BotMessage::MakePostStream => {
                        let _ = tx.send(format!("Starting post stream! 🎈"));
                        TGState::MakePostStream
                    },
                    BotMessage::MakePost { title, tags, link, caption } => todo!(),
                    _ => {
                        let _ = tx.send(format!("I don't know what to do with `{:?}`. 😥", parsed));
                        TGState::Start
                    }
                }
            },
            TGState::MakePostStream => {
                if let BotMessage::Done = parsed {
                    let _ = tx.send("Post stream ended. 🤖".to_string());
                    TGState::Start
                } else {
                    let parsed = parse_make_post(&message.text);
                    let _ = tx.send(format!("Post: `{:?}`. 😥", parsed));
                    TGState::MakePostStream
                }
            }
        },
        Err(parsed) => {
            let _ = tx.send(format!("{:?}", parsed));
            state
        }
    }
}
