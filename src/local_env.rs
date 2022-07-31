use core::panic;
use std::{net::Ipv4Addr, env, str::FromStr};
use log::{error, warn, info, debug, trace, LevelFilter};

use lazy_static::lazy_static;

fn var_not_defined(var: &str) -> String {
    format!("[{}] -- {} environment variable not defined", "Main", var)
}

pub fn check_vars() {
    lazy_static::initialize(&HOST);
    lazy_static::initialize(&PORT);
    lazy_static::initialize(&DB_PASSWORD);
    lazy_static::initialize(&DB_USERNAME);
    lazy_static::initialize(&DB_HOST);
    lazy_static::initialize(&DB_PORT);
    lazy_static::initialize(&DB_DATABASE);
    lazy_static::initialize(&DB_PARAMS);
    lazy_static::initialize(&REDIS_HOST);
    lazy_static::initialize(&REDIS_PORT);
}

lazy_static! {
    /// Service
    pub static ref HOST: Ipv4Addr = Ipv4Addr::from_str(env::var("HOST").unwrap_or_else(|e| {
        panic!("{}", var_not_defined("HOST"));
    }).as_str()).unwrap();
    pub static ref PORT: u16 = env::var("PORT").unwrap_or_else(|e| {
        panic!("{}", var_not_defined("HOST"));
    }).parse().unwrap_or_else(|e| {
        panic!("Can't parse PORT {}", e);
    });

    /// Database
    pub static ref DB_PASSWORD: String = env::var("DB_PASSWORD").unwrap_or_else(|e| {
        panic!("{}", var_not_defined("DB_PASSWORD"));
    });
    pub static ref DB_USERNAME: String = env::var("DB_USERNAME").unwrap_or_else(|e| {
        panic!("{}", var_not_defined("DB_USERNAME"));
    });
    pub static ref DB_HOST: String = env::var("DB_HOST").unwrap_or_else(|e| {
        panic!("{}", var_not_defined("DB_HOST"));
    });
    pub static ref DB_PORT: u16 = env::var("DB_PORT").unwrap_or_else(|e| {
        panic!("{}", var_not_defined("DB_PORT"));
    }).parse().unwrap_or_else(|e| {
        panic!("Can't parse DB_PORT {}", e);
    });

    pub static ref DB_DATABASE: String = env::var("DB_DATABASE").unwrap_or_else(|e| {
        panic!("{}", var_not_defined("DB_DATABASE"));
    });
    pub static ref DB_PARAMS: String = env::var("DB_PARAMS").unwrap_or_else(|e| {
        panic!("{}", var_not_defined("DB_PARAMS"));
    });

    /// Redis
    pub static ref REDIS_HOST: String = env::var("REDIS_HOST").unwrap_or_else(|e| {
        panic!("{}", var_not_defined("REDIS_HOST"));
    });
    pub static ref REDIS_PORT: u16 = env::var("REDIS_PORT").unwrap_or_else(|e| {
        panic!("{}", var_not_defined("REDIS_PORT"));
    }).parse().unwrap_or_else(|e| {
        panic!("Can't parse REDIS_PORT {}", e);
    });

}
