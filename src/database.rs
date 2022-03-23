use sqlx::{mysql::MySqlPoolOptions, MySqlPool};
use std::env;

pub fn create_connection_pool() -> MySqlPool {
    MySqlPoolOptions::new()
        .connect_lazy(&env::var("DATABASE_URL").unwrap()).unwrap()
}
