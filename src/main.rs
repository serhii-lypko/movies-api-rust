use actix_web::web::Data;
use actix_web::{get, web, App, HttpResponse, HttpServer};
use std::env;

#[macro_use]
extern crate diesel;

// use diesel::prelude::*;

mod api_error;
mod db;
mod models;
mod routes;
mod schema;

use crate::db::create_connection_pool;
use crate::routes::{create_movie, delete_movie, get_movies};

/* --- --- --- --- --- --- --- --- --- --- --- --- --- --- */

#[get("/")]
async fn ok() -> HttpResponse {
    HttpResponse::Ok().finish()
}

fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(get_movies);
    cfg.service(create_movie);
    cfg.service(delete_movie);
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();

    let database_url = env::var("DATABASE_URL").unwrap();
    let db_connection_pool = create_connection_pool(&database_url);

    // let movie_service = MovieService::create(db_connection_pool.clone());

    HttpServer::new(move || {
        App::new()
            // .app_data(movie_service.clone())
            .app_data(Data::new(db_connection_pool.clone())) // why wrap with Data?
            .service(ok)
            .configure(config)
    })
    .bind(("127.0.0.1", 8000))?
    .run()
    .await
}
