use dotenv::dotenv;
use futures::stream::Stream;
use std::env;
use telebot::Bot;

use telebot::functions::*;

fn main() {
    // Load the environment variables from the .env file
    dotenv().ok();

    // Create the bot
    let mut bot = Bot::new(&env::var("TELEGRAM_BOT_KEY").unwrap()).update_interval(200);

    // Register a reply command which answers a message
    let handle = bot
        .new_cmd("/reply")
        .and_then(|(bot, msg)| {
            let mut text = msg.text.unwrap().clone();
            if text.is_empty() {
                text = "Hey! Write me something!".into();
            }

            bot.message(msg.chat.id, text).send()
        })
        .for_each(|_| Ok(()));

    bot.run_with(handle);
}
