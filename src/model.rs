#[derive(Debug)]
pub struct Tote {
    pub id: i32,
    pub note: String,
    pub created_at: mysql::chrono::NaiveDateTime,
    pub updated_at: mysql::chrono::NaiveDateTime
}