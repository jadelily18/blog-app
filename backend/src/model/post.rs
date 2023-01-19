use serde::{Serialize, Deserialize};
use sqlx::FromRow;

#[derive(FromRow, Serialize, Deserialize)]
pub struct Post {
    pub post_id: i64,
    pub date_time: Option<String>,
    pub author_id: i64,
    pub title: String,
    pub description: Option<String>,
    pub content: String
}

#[derive(Serialize, Deserialize)]
pub struct PostList {
    pub posts: Vec<Post>
}

#[derive(Serialize, Deserialize)]
pub struct NewPost {
    pub date_time: Option<String>,
    pub author_id: i64,
    pub title: String,
    pub description: Option<String>,
    pub content: String,
}
