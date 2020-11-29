use chrono::NaiveDateTime;

#[derive(Queryable)]
pub struct Mention {
    pub user_id: i32,
    pub updated_at: NaiveDateTime,
}
