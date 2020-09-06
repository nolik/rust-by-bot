use std::env;

use chrono::{DateTime, NaiveDateTime, Utc};
use futures::StreamExt;
use regex::Regex;
use telegram_bot::*;

#[tokio::main]
async fn main() -> Result<(), Error> {
    let token = env::var("TELEGRAM_BOT_TOKEN").expect("TELEGRAM_BOT_TOKEN not set");
    let api = Api::new(token);
    let re = Regex::new(r"Rust|rust|Раст|раст").unwrap();
    let mut last_date: DateTime<Utc> = Utc::now();

    // Fetch new updates via long poll method
    let mut stream = api.stream();
    while let Some(update) = stream.next().await {
        // If the received update contains a new message...
        let update = update?;
        if let UpdateKind::Message(message) = update.kind {
            if let MessageKind::Text { ref data, .. } = message.kind {
                // Print received text message to stdout.
                println!("<{}>: {}", &message.from.first_name, data);
                println!("date: <{}>", &message.date);

                let curr_native_dt = NaiveDateTime::from_timestamp(*&message.date, 0);
                let curr_date = DateTime::from_utc(curr_native_dt, Utc);
                let diff = curr_date.signed_duration_since(last_date);

                if re.is_match(data) {
                    api.send(message.text_reply(format!(
                        "Hi, {}! You just wrote smth about Rust! \nBe careful, \
                         {} science last incident.",
                        &message.from.first_name, diff
                    )))
                    .await?;

                    last_date = curr_date;
                }
            }
        }
    }
    Ok(())
}
