use teloxide::prelude::*;
use std::env;

const MIKE_CHAT_ID: i64 = 248923795;

pub async fn run_telegram_bot(){
    let token = env::var("TELEGRAM_BOT_ACCESS_TOKEN").expect("TELEGRAM_BOT_ACCESS_TOKEN not set in environment");
    let bot = Bot::new(token);

    let _ = bot.send_message(ChatId(MIKE_CHAT_ID), "TELEGRAM BOT STARTED").await;

    teloxide::repl(bot, |bot: Bot, msg: Message| async move {
        println!("{:?}", msg);
        // bot.send_dice(msg.chat.id).await?;
        if let Some(txt) = msg.text() {
            let _ = bot.send_message(msg.chat.id, txt).await;
        }
        Ok(())
    }).await;
}