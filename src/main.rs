use actix_web::cookie::Key;
use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder, guard, middleware, error};
use actix_identity::IdentityMiddleware;
use actix_session::{storage::RedisSessionStore, SessionMiddleware};
use log::{error, warn, info, debug, trace, LevelFilter};
use env_logger;
use dotenv::dotenv;
use lazy_static::lazy_static;
use openssl::ssl::{SslAcceptor, SslFiletype, SslMethod};
use std::env;
use std::io::Write;
use std::fmt::{Display, Debug};
use std::net::{Ipv4Addr, SocketAddr, SocketAddrV4};
use std::str::FromStr;
use local_env::*;

mod hashing;
mod database;
mod schema;
mod models;
mod local_env;

mod health;
mod users;

use users::users_config;

pub struct AppState {
    app_name: String,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    env_logger::Builder::new()
        .format(|buf, record| writeln!(buf, "[{}] - {}", record.level(), record.args()))
        .filter(None, LevelFilter::Info)
        .target(env_logger::Target::Stdout)
        .write_style(env_logger::fmt::WriteStyle::Always)
        .init();

    local_env::check_vars();

    info!("[{}] -- Starting server..", "Main");
    info!("[{}] -- Host: {} Port {}", "Main", *HOST, *PORT);

    // TODO: https ON/OFF
    // TODO: store key elsewhere
    // openssl req -x509 -newkey rsa:4096 -nodes -keyout key.pem -out cert.pem -days 365 -subj '/CN=localhost'
    let mut builder = SslAcceptor::mozilla_intermediate(SslMethod::tls()).unwrap();
    builder
        .set_private_key_file("key.pem", SslFiletype::PEM)
        .unwrap();
    builder.set_certificate_chain_file("cert.pem").unwrap();

    let secret_key = Key::generate(); // TODO: store key elsewhere
    let redis_connection_string = format!("redis://{}:{}", &REDIS_HOST.as_str(), &REDIS_PORT.to_string());
    let store = RedisSessionStore::new(redis_connection_string).await.unwrap();

    let socket = SocketAddrV4::new(*HOST, *PORT);
    HttpServer::new(move || {
        let json_cfg = web::JsonConfig::default()
        // limit request payload size
        .limit(4096)
        // only accept text/plain content type
        .content_type(|mime| mime == mime::APPLICATION_JSON)
        // use custom error handler
        .error_handler(|err, req| {
            // println!("[{}] -- Error: {}", "Main", err);
            error!("[{}] - [{}] -- {}", "Main", req.path(), err);
            error::InternalError::from_response(err, HttpResponse::BadRequest().into()).into()
        });
        
        App::new()
            .wrap(IdentityMiddleware::default())
            .wrap(SessionMiddleware::new(
                store.clone(),
                secret_key.clone()
            ))
            .wrap(middleware::Compress::default())
            .app_data(json_cfg)
            .app_data(web::Data::new(AppState {
                app_name: String::from("Actix Web"),
            }))
            .route("/health", web::get().to(health::check))
            .service(
                web::scope("/users").configure(users_config)
            )
    })
    .bind_openssl(socket, builder)?
    // .bind(socket)?
    .run()
    .await
}
