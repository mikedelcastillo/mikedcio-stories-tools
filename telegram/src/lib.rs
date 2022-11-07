use teloxide::prelude::*;
use std::env;

use utils;

const MIKE_CHAT_ID: i64 = 248923795;

pub async fn run_telegram_bot(){
    let token = env::var("TELEGRAM_BOT_ACCESS_TOKEN").expect("TELEGRAM_BOT_ACCESS_TOKEN not set in environment");
    let bot = Bot::new(token);

    let _ = bot.send_message(ChatId(MIKE_CHAT_ID), "TELEGRAM BOT STARTED").await;

    teloxide::repl(bot, |bot: Bot, msg: Message| async move {

        let txt = if let Some(txt) = msg.text() {
            txt
        } else if let Some(txt) = msg.caption() {
            txt
        } else{
            ""
        };

        
        let command = utils::parse_bot_message(txt);
        
        let response = match command {
            Ok(command) => format!("command is: {:?}", command),
            Err(err) => format!("command error: {:?}", err),
        };

        let response = format!("{}\n\n{:?}", response, msg);

        let _ = bot.send_message(msg.chat.id, response).await;
        Ok(())
    }).await;
}