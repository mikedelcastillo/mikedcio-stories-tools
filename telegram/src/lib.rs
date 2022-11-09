use std::{
    sync::{mpsc},
    thread,
};

mod api;
use api::TGApi;

#[derive(Debug)]
pub enum TGState {
    Start,
}

pub fn run_telegram_bot() {
    let (tx, rx) = mpsc::channel::<String>();

    let tx_thread = thread::spawn(move || {
        println!("TG tx thread started.");
        let api = TGApi::new_from_env();
        crossbeam::thread::scope(|s| {
            for message in rx {
                s.spawn(|_| {
                    let _ = api.send(message);
                });
            }
        })
    });

    let rx_thread = thread::spawn(move || {
        println!("TG rx thread started.");
        let mut api = TGApi::new_from_env();

        let _ = tx.send("Bot ready. Share stories about your day! 🤖".to_string());

        loop {
            match api.get_updates() {
                Err(err) => println!("{:?}", err),
                Ok(messages) => for message in messages {
                    let mtx = tx.clone();

                    let _ = mtx.send(format!("{:?}", message));
                },
            }
        }
    });

    let _ = tx_thread.join().unwrap();
    let _ = rx_thread.join().unwrap();
}
