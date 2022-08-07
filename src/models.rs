use crate::schema::movie;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use diesel::prelude::*;

use crate::api_error::ApiError;
use crate::db::DatabasePool;

/* --- --- --- --- --- --- --- --- --- --- --- --- --- --- */

#[derive(Serialize, Deserialize, Queryable, Insertable, Debug, Clone)]
#[table_name = "movie"]
pub struct Movie {
    pub id: Uuid,
    pub title: String,
    pub director: String,
}

#[derive(Serialize, Deserialize, Queryable, AsChangeset)]
#[table_name = "movie"]
pub struct NewMovie {
    pub title: String,
    pub director: String,
}

impl From<NewMovie> for Movie {
    fn from(movie: NewMovie) -> Self {
        Movie {
            id: Uuid::new_v4(),
            title: movie.title,
            director: movie.director,
        }
    }
}

pub trait Crud<I> {
    fn list(&self) -> Result<Vec<I>, ApiError>;
    fn insert(&self, item: I) -> Result<I, ApiError>;
}

impl Crud<Movie> for DatabasePool {
    fn list(&self) -> Result<Vec<Movie>, ApiError> {
        let movies = movie::table.load::<Movie>(&self.get().unwrap())?;

        Ok(movies)
    }

    fn insert(&self, item: Movie) -> Result<Movie, ApiError> {
        let movie: Movie = diesel::insert_into(movie::table)
            .values(item)
            // TODO: how to avoid having unwrap()
            .get_result(&self.get().unwrap())?;

        Ok(movie)
    }
}
