use actix_web::{web, get, HttpResponse};
use sqlx::Sqlite;

use crate::model::post::{Post, PostList};
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
pub async fn get_post_from_id(
    pool: web::Data<sqlx::Pool<Sqlite>>,
    post_id: web::Path<String>) -> Result<HttpResponse, ApiError> {

    let data = sqlx::query_as::<_, Post>(
        &format!(
            "SELECT * FROM posts WHERE post_id = '{}';", post_id)
    )
        .bind(150_i64)
        .fetch_one(&**pool).await;

    Ok(HttpResponse::Ok().json(data.unwrap()))

}
