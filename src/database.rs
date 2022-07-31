use std::{
    env,
};
use lazy_static::lazy_static;
use dotenv::dotenv;
use diesel::{
    pg::PgConnection,
    // mysql::MysqlConnection,
    r2d2::{
        ConnectionManager,
        Pool,
    }
};
use crate::local_env::*;

pub type QueryResult<T> = Result<T, diesel::result::Error>;

lazy_static! {
    pub static ref POOL: Pool<ConnectionManager<PgConnection>> = {
        dotenv().ok();
        let database_url = format!(
            "postgres://{}:{}@{}:{}/{}{}",
            *DB_PASSWORD,
            *DB_USERNAME,
            *DB_HOST,
            *DB_PORT,
            *DB_DATABASE,
            *DB_PARAMS
        );

        let manager = ConnectionManager::<PgConnection>::new(database_url);
        diesel::r2d2::Pool::builder()
            .test_on_check_out(true)
            .max_size(10)
            .build(manager)
            .unwrap()
    };
}