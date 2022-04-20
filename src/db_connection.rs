use diesel::r2d2::{ConnectionManager, Pool, PoolError, PooledConnection};
use diesel::{Connection, PgConnection};
use dotenv::dotenv;
use lazy_static::lazy_static;

use crate::config::get_config;

pub type PgPool = Pool<ConnectionManager<PgConnection>>;
pub type PgPooledConnection = PooledConnection<ConnectionManager<PgConnection>>;

fn init_pool(database_url: &str) -> Result<PgPool, PoolError> {
    let manager = ConnectionManager::<PgConnection>::new(database_url);
    Pool::builder().build(manager)
}

pub fn establish_connection() -> PgPool {
    embed_migrations!();
    dotenv().ok();

    let database_url = get_config().db_url;
    let connection = PgConnection::establish(&database_url)
        .expect(&format!("Error connecting to {}", database_url));
    embedded_migrations::run_with_output(&connection, &mut std::io::stdout());

    init_pool(&database_url).expect("Failed to create pool")
}

pub fn pg_pool_handler(pool: &PgPool) -> Result<PgPooledConnection, PoolError> {
    let _pool = pool.get().unwrap();
    Ok(_pool)
}

lazy_static! {
    pub static ref DB_POOL: PgPool = establish_connection();
}
