use crate::schema::mentions;
use chrono::NaiveDateTime;

#[derive(Queryable)]
pub struct Mention {
    pub user_id: i64,
    pub updated_at: NaiveDateTime,
}

#[derive(Insertable)]
#[table_name = "mentions"]
pub struct NewMention<'a> {
    pub user_id: &'a i64,
}
