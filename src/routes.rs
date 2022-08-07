use actix_web::web::{Data, Json};
use actix_web::{HttpResponse, Result};

use crate::api_error::ApiError;
use crate::db::DatabasePool;
use crate::models::{Crud, Movie, NewMovie};

/* --- --- --- --- --- --- --- --- --- --- --- --- --- --- */

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

pub async fn get_movies(database: Data<DatabasePool>) -> Result<HttpResponse, ApiError> {
    let movies: Vec<Movie> = database.list()?;

    let first_movie = &movies[0];

    println!("{}", first_movie);

    Ok(HttpResponse::Ok().json(movies))
}
