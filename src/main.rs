use std::env;

use futures::StreamExt;
use regex::Regex;
use telegram_bot::*;

#[tokio::main]
async fn main() -> Result<(), Error> {

    let token = env::var("TELEGRAM_BOT_TOKEN").expect("TELEGRAM_BOT_TOKEN not set");
    let api = Api::new(token);
    let re = Regex::new(r"Rust|rust|Раст|раст").unwrap();

    // Fetch new updates via long poll method
    let mut stream = api.stream();
    while let Some(update) = stream.next().await {
        // If the received update contains a new message...
        let update = update?;
        if let UpdateKind::Message(message) = update.kind {
            if let MessageKind::Text { ref data, .. } = message.kind {
                // Print received text message to stdout.
                println!("<{}>: {}", &message.from.first_name, data);

                if re.is_match(data) {
                    api.send(message.text_reply(format!(
                        "Hi, {}! You just wrote smth about Rust!", &message.from.first_name)))
                        .await?;
                }
            }
        }
    }
    Ok(())
}
