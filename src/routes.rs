use actix_web::web::{Data, Json, Path};
use actix_web::{delete, get, post, put};
use actix_web::{HttpResponse, Responder, Result};
use uuid::Uuid;

use crate::api_error::ApiError;
use crate::db::DatabasePool;
use crate::models::{Crud, Movie, NewMovie};

/* --- --- --- --- --- --- --- --- --- --- --- --- --- --- */

#[get("/movies")]
pub async fn get_movies(database: Data<DatabasePool>) -> Result<HttpResponse, ApiError> {
    let movies: Vec<Movie> = database.list()?;

    // let first_movie = &movies[0];

    // println!("{}", first_movie);

    Ok(HttpResponse::Ok().json(movies))
}

#[post("/movie")]
pub async fn create_movie(
    movie_json: Json<NewMovie>,
    database: Data<DatabasePool>,
) -> Result<HttpResponse, ApiError> {
    // TODO: why operations are not async?

    // let movies: Vec<Movie> = database.list()?;
    // let next_index = movies.len() + 1;

    let new_movie = movie_json.into_inner();
    let movie = Movie::from(new_movie);

    let saved_movie = database.insert(movie)?;

    Ok(HttpResponse::Ok().json(saved_movie))
}

#[delete("/movie/{id}")]
pub async fn delete_movie(id: Path<Uuid>, database: Data<DatabasePool>) -> impl Responder {
    // TODO: how database service knows about exact model?
    database.delete(id.into_inner()).unwrap();

    HttpResponse::Ok()
}
