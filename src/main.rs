use dotenv::dotenv;
use futures::stream::Stream;
use futures::Future;
use reqwest::blocking::Client;
use std::collections::HashMap;
use std::env;
use telebot::functions::*;
use telebot::Bot;

fn main() {
    // Load the environment variables from the .env file
    dotenv().ok();

    // Create the bot
    let mut bot = Bot::new(&env::var("TELEGRAM_BOT_KEY").unwrap()).update_interval(200);

    // Register a reply command which answers a message
    let health_handle = bot
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

    let help_handle = bot
        .new_cmd("/help")
        .and_then(|(bot, msg)| {
            let mut commands = HashMap::new();
            commands.insert("/health", "Check the health of the INCTAGRAM server");

            let mut text: String = "Available commands:\n".into();
            for (cmd, description) in &commands {
                text.push_str(&format!("<b>{}</b> - {}\n", cmd, description));
            }

            bot.message(msg.chat.id, text)
                .parse_mode(ParseMode::HTML)
                .send()
        })
        .for_each(|_| Ok(()));

    bot.run_with(health_handle.join(help_handle));
}
