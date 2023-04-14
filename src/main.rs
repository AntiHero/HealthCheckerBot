use dotenv::dotenv;
use futures::stream::Stream;
use reqwest::blocking::Client;
use std::env;
use telebot::functions::*;
use telebot::Bot;

fn main() {
    // Load the environment variables from the .env file
    dotenv().ok();

    // Create the bot
    let mut bot = Bot::new(&env::var("TELEGRAM_BOT_KEY").unwrap()).update_interval(200);

    // Register a reply command which answers a message
    let handle = bot
        .new_cmd("/health")
        .and_then(|(bot, msg)| {
            let client = Client::new();

            let mut text = "<b>Dead</b>".into();

            let ai_response = client
                .get("https://inctagram.herokuapp.com/health-check")
                .send();

            if ai_response.is_ok() {
                let status = ai_response.as_ref().unwrap().status();
                if status.is_success() {
                    text = "<b>Alive</b>".into();
                }
            }

            bot.message(msg.chat.id, text)
                .parse_mode(ParseMode::HTML)
                .send()
        })
        .for_each(|_| Ok(()));

    bot.run_with(handle);
}
