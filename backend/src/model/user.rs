use serde::{Serialize, Deserialize};
use sqlx::FromRow;

#[derive(FromRow, Serialize, Deserialize)]
pub struct User {
    pub user_id: i64,
    pub first_name: String,
    pub last_name: Option<String>,
    pub avatar_url: Option<String>
}

#[derive(Serialize, Deserialize)]
pub struct UserList {
    pub users: Vec<User>
}
