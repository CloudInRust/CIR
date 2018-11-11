use std::env;
use dotenv::dotenv;
use diesel::prelude::*;
use diesel::r2d2::ConnectionManager;
use r2d2::Pool;

mod db_actor;
pub mod models;
mod schema;

pub use self::db_actor::DbExecutor;

pub type MysqlPool = Pool<ConnectionManager<MysqlConnection>>;
pub type DBConn = MysqlConnection;

pub struct DBSubsystem {
    pool: MysqlPool
}

impl DBSubsystem {
    pub fn init() -> DBSubsystem {
        dotenv().expect("Unable to load .env configuration");

        let db_url = env::var("DATABASE_URL")
            .expect("DATABASE_URL not set");

        // Start the connection pool
        let manager = ConnectionManager::<MysqlConnection>::new(db_url);
        let pool = Pool::builder()
            .build(manager)
            .expect("Failed to create DB Connection Pool");

        DBSubsystem {
            pool
        }
    }

    pub fn clone_pool(&self) -> MysqlPool {
        self.pool.clone()
    }
}