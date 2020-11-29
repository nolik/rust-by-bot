#[macro_use]
extern crate diesel;

mod connection;
mod mentions;
mod schema;

use std::env;

use crate::connection::establish_connection;
use crate::mentions::Mention;
use crate::schema::mentions::dsl::*;
use chrono::{DateTime, Duration, NaiveDateTime, Utc};
use diesel::RunQueryDsl;
use futures::StreamExt;
use regex::Regex;
use std::ops::Sub;
use telegram_bot::*;

const HOURS_PER_DAY: i64 = 24;
const MINUTES_PER_HOUR: i64 = 60;

#[tokio::main]
async fn main() -> Result<(), Error> {
    let token = env::var("TELEGRAM_BOT_TOKEN_TEST").expect("TELEGRAM_BOT_TOKEN not set");
    let api = Api::new(token);
    let re = Regex::new(r"\b[RrРр][AaUuАа][CcSsСс][TtТт]\b").unwrap();
    let min_time_diff = Duration::minutes(15);
    let mut last_date: DateTime<Utc> = Utc::now().sub(Duration::days(2));
    let connection = establish_connection();

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

                    // Make a SQL request to load all mentions
                    let results = mentions
                        .load::<Mention>(&connection)
                        .expect("Error loading mentions");
                    for mention in results {
                        println!("{}", mention.user_id)
                    }

                    if time_diff > min_time_diff {
                        api.send(message.text_reply(format!(
                            "Hi, {}! You just wrote smth about Rust! \nBe careful, \
                         {}d:{}h:{}m since last incident.",
                            &message.from.first_name,
                            time_diff.num_days(),
                            time_diff.num_hours() % HOURS_PER_DAY,
                            time_diff.num_minutes() % MINUTES_PER_HOUR
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
