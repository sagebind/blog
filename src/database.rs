use mysql::{Pool, Opts};
use std::env;

pub fn create_connection_pool() -> Pool {
    let opts = Opts::from_url(&env::var("DATABASE_URL").unwrap()).unwrap();
    Pool::new(opts).unwrap()
}
