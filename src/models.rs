use std::fmt;

use crate::schema::movies;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use diesel::prelude::*;

use crate::api_error::ApiError;
use crate::db::DatabasePool;

/* --- --- --- --- --- --- --- --- --- --- --- --- --- --- */

#[derive(Serialize, Deserialize, Queryable, Insertable, Debug, Clone)]
#[table_name = "movies"]
pub struct Movie {
    id: Uuid,
    title: String,
    director: String,
    year: i32,
    author: String,
}

impl fmt::Display for Movie {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "(Title: {}, Director: {}, Year: {}, Author {})",
            self.title, self.director, self.year, self.author
        )
    }
}

#[derive(Serialize, Deserialize, AsChangeset)]
#[table_name = "movies"]
pub struct NewMovie {
    title: String,
    director: String,
    year: i32,
    author: String,
}

impl From<NewMovie> for Movie {
    fn from(movie: NewMovie) -> Self {
        Movie {
            id: Uuid::new_v4(),
            title: movie.title,
            director: movie.director,
            year: movie.year,
            author: movie.author,
        }
    }
}

pub trait Crud<I> {
    fn list(&self) -> Result<Vec<I>, ApiError>;
    fn insert(&self, item: I) -> Result<I, ApiError>;
    fn delete(&self, id: Uuid) -> Result<(), ApiError>;
}

impl Crud<Movie> for DatabasePool {
    fn list(&self) -> Result<Vec<Movie>, ApiError> {
        let movies = movies::table.load::<Movie>(&self.get().unwrap())?;

        Ok(movies)
    }

    fn insert(&self, item: Movie) -> Result<Movie, ApiError> {
        let movie: Movie = diesel::insert_into(movies::table)
            .values(item)
            // TODO: how to avoid having unwrap()
            .get_result(&self.get().unwrap())?;

        Ok(movie)
    }

    fn delete(&self, id: Uuid) -> Result<(), ApiError> {
        diesel::delete(movies::table.filter(movies::id.eq(id))).execute(&self.get().unwrap())?;

        Ok(())
    }
}
