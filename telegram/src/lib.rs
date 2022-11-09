use std::{
    sync::{Arc, Mutex},
    thread,
    time::Duration,
};

mod api;
use api::TGApi;

#[derive(Debug)]
pub enum TGState {
    Start,
}

pub fn run_telegram_bot() {
    // let mut api = TGApi::new(&token, &admin_chat_id);

    // let _ = api.send("Bot ready. Share stories about your day! 🤖".to_string());

    // loop {
    //     match api.get_updates() {
    //         Ok(_) => (),
    //         Err(err) => println!("Could not get updates. {}", err),
    //     };
    // }

    let outgoing = Arc::new(Mutex::new(vec![
        "Bot ready. Share stories about your day! 🤖".to_string(),
    ]));

    let shared_outgoing = outgoing.clone();
    let tx_thread = thread::spawn(move || {
        let api = TGApi::new_from_env();
        loop {
            let mut outgoing = shared_outgoing.lock().unwrap();
            println!("t{:?}", outgoing);
            let _ = api.send_multiple(outgoing.clone());
            outgoing.clear();
            std::mem::drop(outgoing);
            thread::sleep(Duration::from_millis(100))
        }
    });

    let shared_outgoing = outgoing.clone();
    let rx_thread = thread::spawn(move || {
        // let mut api = TGApi::new_from_env();
        loop {
            let mut outgoing = shared_outgoing.lock().unwrap();
            outgoing.push("this is a random message...".to_string());
            println!("r{:?}", outgoing.clone());
            std::mem::drop(outgoing);
            thread::sleep(Duration::from_millis(1000))
        }
    });

    tx_thread.join().unwrap();
    rx_thread.join().unwrap();
}
