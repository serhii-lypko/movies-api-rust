use actix_web::web::{Data, Json};
use actix_web::{get, web, App, HttpResponse, HttpServer, Responder, Result};
use std::env;

#[macro_use]
extern crate diesel;

// use diesel::prelude::*;

mod api_error;
mod db;
mod models;
mod schema;

use crate::api_error::ApiError;
use crate::db::{create_connection_pool, DatabasePool};
use crate::models::{Crud, Movie, NewMovie};

/* --- --- --- --- --- --- --- --- --- --- --- --- --- --- */

// struct AppState {
//     app_name: String,
// }

// struct MovieService {
//     db_connection_pool: DatabasePool,
// }

// impl MovieService {
//     pub fn create(db_connection_pool: DatabasePool) -> web::Data<Self> {
//         let service = Data::new(Self::new(db_connection_pool));
//         service.clone()
//     }

//     fn new(db_connection_pool: DatabasePool) -> Self {
//         Self { db_connection_pool }
//     }

//     pub fn create_movie(&self, movie: Json<NewMovie>) -> Result<Movie, ApiError> {
//         // TODO: how to refator this without _result?

//         let new_movie = movie.into_inner();
//         let movie = Movie::from(new_movie);

//         self.db_connection_pool.insert(movie)
//     }
// }

#[get("/")]
async fn ok() -> HttpResponse {
    HttpResponse::Ok().finish()
}

async fn create_movie_handler(
    movie_json: Json<NewMovie>,
    database: Data<DatabasePool>,
) -> Result<HttpResponse, ApiError> {
    // TODO: why operations are not async?

    let new_movie = movie_json.into_inner();
    let movie = Movie::from(new_movie);

    let saved_movie = database.insert(movie)?;

    Ok(HttpResponse::Ok().json(saved_movie))
}

async fn get_movies_handler(database: Data<DatabasePool>) -> Result<HttpResponse, ApiError> {
    let movies: Vec<Movie> = database.list()?;

    Ok(HttpResponse::Ok().json(movies))
}

fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(web::resource("/movies").route(web::get().to(get_movies_handler)));
    cfg.service(web::resource("/create-movie").route(web::post().to(create_movie_handler)));
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
