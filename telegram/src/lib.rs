use std::env;

mod api;
use api::TGApi;

#[derive(Debug)]
pub enum TGState {
    Start,
}

pub fn run_telegram_bot() {
    let token = env::var("TELEGRAM_BOT_ACCESS_TOKEN")
        .expect("TELEGRAM_BOT_ACCESS_TOKEN not set in environment");
    let admin_chat_id =
        env::var("TELEGRAM_ADMIN_CHAT_ID").expect("TELEGRAM_ADMIN_CHAT_ID not set in environment");

    let mut api = TGApi::new(&token, &admin_chat_id);

    let _ = api.send("Bot ready. Share stories about your day! 🤖".to_string());

    loop {
        match api.get_updates() {
            Ok(_) => (),
            Err(err) => println!("Could not get updates. {}", err),
        };
    }
}
