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
