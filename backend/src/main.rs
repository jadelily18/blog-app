use actix_web::{App, get, HttpResponse, HttpServer, Responder, web};

use routes::user;
use routes::post;

mod database;
mod routes;
mod error;
pub mod model;

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("hiya")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenvy::dotenv().expect(".env file not found");

    let pool = database::connect()
        .await.expect("Couldn't connect to database");

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .service(
            web::scope("api")
                .service(hello)
                .service(user::get_all_users)
                .service(post::get_all_posts)
                .service(
                    web::scope("user")
                        .service(user::get_user_by_id)
                )
                .service(
                    web::scope("post")
                        .service(post::get_post_by_id)
                )
                .service(
                    web::scope("create")
                        .service(post::create_post)
                )
            )

    })
        .bind((
            dotenvy::var("BIND_ADDR").unwrap(),
            dotenvy::var("BIND_PORT").unwrap().parse().unwrap()
        ))?
        .run()
        .await
}
