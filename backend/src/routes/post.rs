use actix_web::{web, get, post, HttpResponse};
use chrono::{Utc};
use sqlx::Sqlite;

use crate::model::post::{NewPost, Post, PostList};
use crate::error::ApiError;

#[get("posts")]
pub async fn get_all_posts(pool: web::Data<sqlx::Pool<Sqlite>>) -> Result<HttpResponse, ApiError> {

    let data = sqlx::query_as::<_, Post>("SELECT * FROM posts;")
        .bind(150_i64)
        .fetch_all(&**pool).await;

    let res = PostList {
        posts: data.unwrap()
    };

    Ok(HttpResponse::Ok().json(res))

}

#[get("{post_id}")]
pub async fn get_post_by_id(
    pool: web::Data<sqlx::Pool<Sqlite>>,
    post_id: web::Path<String>
) -> Result<HttpResponse, ApiError> {
    let data = sqlx::query_as::<_, Post>(
        &format!(
            "SELECT * FROM posts WHERE post_id = '{}';", post_id)
    )
        .bind(150_i64)
        .fetch_one(&**pool).await;

    Ok(HttpResponse::Ok().json(data.unwrap()))
}

#[post("post")]
pub async fn create_post(
    pool: web::Data<sqlx::Pool<Sqlite>>,
    json: web::Json<NewPost>
) -> Result<HttpResponse, ApiError> {

    // Generates epoch timestamp (e.x. 1673920429)
    let date_time = Utc::now().timestamp().to_string();

    sqlx::query!(
        "
        INSERT INTO posts (date_time, author_id, title, description, content)
        VALUES ($1, $2, $3, $4, $5);
        ",
        date_time, json.author_id, json.title, json.description, json.content
    ).execute(&**pool).await?;

    Ok(HttpResponse::Ok().body("Successfully created post!"))
}
