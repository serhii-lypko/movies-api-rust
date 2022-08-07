use diesel::pg::PgConnection;
use diesel::r2d2::{ConnectionManager, Pool};

pub type DatabasePool = Pool<ConnectionManager<PgConnection>>;

pub fn create_connection_pool(database_url: &str) -> DatabasePool {
    let manager = ConnectionManager::<PgConnection>::new(database_url);

    Pool::new(manager).expect(&format!("Error connecting to {}", database_url))
}
