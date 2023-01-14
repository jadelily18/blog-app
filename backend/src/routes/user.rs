use actix_web::{web, get, HttpResponse};
use sqlx::Sqlite;

use crate::model::user::{User, UserList};
use crate::error::ApiError;

// returns all users (e.x. https://example.com/api/users)
#[get("users")]
async fn get_all_users(pool: web::Data<sqlx::Pool<Sqlite>>) -> Result<HttpResponse, ApiError> {

    let data = sqlx::query_as::<_, User>("SELECT * FROM users;")
        .bind(150_i64)
        .fetch_all(&**pool).await;

    let res = UserList {
        users: data.unwrap()
    };

    Ok(HttpResponse::Ok().json(res))
}

#[get("{user_id}")]
async fn get_user_by_id(
    pool: web::Data<sqlx::Pool<Sqlite>>,
    user_id: web::Path<String>) -> Result<HttpResponse, ApiError> {

    let data = sqlx::query_as::<_, User>(
        &format!(
            "SELECT * FROM users WHERE user_id = '{}';", user_id)
    )
        .bind(150_i64)
        .fetch_one(&**pool).await;

    Ok(HttpResponse::Ok().json(data.unwrap()))

}

