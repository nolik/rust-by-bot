use crate::models::NewMention;
use crate::schema::mentions;
use crate::schema::mentions::dsl::*;
use chrono::NaiveDateTime;
use diesel::expression::functions::date_and_time::now;
use diesel::pg::PgConnection;
use diesel::prelude::*;
use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl};
use std::env;
use telegram_bot::UserId;

pub fn establish_connection() -> PgConnection {
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}

pub fn lead_earliest_mention_time(connection: &PgConnection) -> NaiveDateTime {
    mentions
        .select(mentions::updated_at)
        .order(mentions::updated_at.desc())
        .first(connection)
        .expect("Error loading earliest mention time")
}

pub fn create_mention(connection: &PgConnection, mention_user_id: UserId) -> () {
    let uid = i64::from(mention_user_id);
    let new_mention = NewMention { user_id: &uid };

    diesel::insert_into(mentions::table)
        .values(&new_mention)
        .on_conflict(mentions::user_id)
        .do_update()
        .set(mentions::updated_at.eq(now))
        .execute(connection)
        .expect("Error upsert mention");
}
