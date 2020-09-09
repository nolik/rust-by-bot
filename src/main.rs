use std::env;

use chrono::{DateTime, Duration, NaiveDateTime, Utc};
use futures::StreamExt;
use regex::Regex;
use telegram_bot::*;

const HOURS_PER_DAY: i64 = 24;
const MINUTES_PER_DAY: i64 = 1440;

#[tokio::main]
async fn main() -> Result<(), Error> {
    let token = env::var("TELEGRAM_BOT_TOKEN").expect("TELEGRAM_BOT_TOKEN not set");
    let api = Api::new(token);
    let re = Regex::new(r"[RrРр]+[\\.\\_\\-\\\\#\\*\\\\\\+\\=]{0,2}[AaUuАа]+[\\.\\_\\-\\\\#\\*\\\\\\+\\=]{0,2}[СсSsСс]+[\\.\\_\\-\\\\#\\*\\\\\\+\\=]{0,2}[TtТт]").unwrap();
    let min_time_diff = Duration::minutes(15);
    let mut last_date: DateTime<Utc> = Utc::now();

    // Fetch new updates via long poll method
    let mut stream = api.stream();
    while let Some(update) = stream.next().await {
        // If the received update contains a new message...
        let update = update?;
        if let UpdateKind::Message(message) = update.kind {
            if let MessageKind::Text { ref data, .. } = message.kind {
                if re.is_match(data) {
                    let curr_native_date = NaiveDateTime::from_timestamp(*&message.date, 0);
                    let curr_date = DateTime::from_utc(curr_native_date, Utc);
                    let time_diff = curr_date.signed_duration_since(last_date);

                    if time_diff > min_time_diff {
                        api.send(message.text_reply(format!(
                            "Hi, {}! You just wrote smth about Rust! \nBe careful, \
                         {}d:{}h:{}m since last incident.",
                            &message.from.first_name,
                            time_diff.num_days(),
                            time_diff.num_hours() % HOURS_PER_DAY,
                            time_diff.num_minutes() % MINUTES_PER_DAY
                        )))
                        .await?;

                        last_date = curr_date;
                    }
                }
            }
        }
    }
    Ok(())
}
