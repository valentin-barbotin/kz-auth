use actix_identity::Identity;
use actix_session::Session;
use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder, HttpRequest, HttpMessage};
use log::{error, warn, info, debug, trace, LevelFilter};
use serde::{Deserialize, Serialize};
use crate::hashing::{
    generate_hash,
    verify_password
};

mod database;
mod session;

pub enum Mode {
    Id(i32),
    Username(String),
    Email(String),
}

#[derive(Deserialize)]
pub struct UserIdentifier {
    id: Option<i32>,
    username: Option<String>,
    email: Option<String>,
}

#[derive(Serialize)]
struct ResUser {
    pub id: i32,
    pub name: String,
    pub email: String,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Deserialize)]
pub struct AuthRequest {
    pub login: String,
    pub password: String,
}

#[derive(Deserialize, Debug)]
pub struct CreateUser {
    username: String,
    email: String,
    password: String,
}

pub fn users_config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::resource("/auth")
            .route(web::post().to(auth))
            .route(web::get().to(HttpResponse::MethodNotAllowed))
    );
    cfg.service(
        web::resource("/list")
            .route(web::get().to(list))
            .route(web::post().to(HttpResponse::MethodNotAllowed))
    );
    cfg.service(
        web::resource("/{id}")
            .route(web::get().to(get_user))
            .route(web::delete().to(delete_user))
            .route(web::post().to(HttpResponse::MethodNotAllowed))
    );
    cfg.service(
        web::resource("/")
            .route(web::post().to(create))
    );
}

fn get_id_from_req(info: web::Path<UserIdentifier>) -> Result<i32, &'static str> {
    if info.id.is_none() {
        error!("[{}] -- No id provided", "UserService::get_id_from_req");
        return Err("No id provided");
    }

    Ok(info.id.unwrap())
}

fn auth_user(provided_password: &[u8], password: &str) -> Result<bool, argon2::password_hash::Error> {
    let res = verify_password(provided_password, password)?;
    Ok(res)
}

pub async fn auth(_req: HttpRequest, body: web::Json<AuthRequest>, sess: Session) -> impl Responder {
    info!("[{}] -- Authenticating user", "UserService::auth");
    let mut user = database::get_user(Mode::Username(body.login.clone())).await;
    match user {
        Ok(user) => {
            let res = auth_user(&body.password.as_bytes(), &user.password);
            match res {
                Ok(true) => {
                    info!("[{}] -- User authenticated", "UserService::auth");
                    info!("[{}] -- Session creation..", "UserService::auth");

                    match session::create_session(&_req.extensions(), user.name.clone()) {
                        Ok(()) => {
                            info!("[{}] -- Session created", "UserService::auth");
                            HttpResponse::Ok().json(user)
                        },
                        Err(e) => {
                            error!("[{}] -- Session creation failed: {}", "UserService::auth", e);
                            HttpResponse::InternalServerError().finish()
                        }
                    }
                },
                Ok(false) => {
                    error!("[{}] -- User authentication failed", "UserService::auth");
                    HttpResponse::Unauthorized().finish()
                },
                Err(e) => {
                    error!("[{}] -- User authentication failed: {}", "UserService::auth", e);
                    HttpResponse::Unauthorized().finish()
                }
            }
        }
        Err(err) => {
            warn!("[{}] -- User authentication failed using username: {}", "UserService::auth", err);
            user = database::get_user(Mode::Email(body.login.clone())).await;
            match user {
                Ok(user) => {
                    let res = auth_user(&body.password.as_bytes(), &user.password);
                    match res {
                        Ok(true) => {
                            info!("[{}] -- User authenticated", "UserService::auth");
                            HttpResponse::Ok().json(user) // TODO
                        },
                        Ok(false) => {
                            error!("[{}] -- User authentication failed", "UserService::auth");
                            HttpResponse::Unauthorized().finish()
                        },
                        Err(e) => {
                            error!("[{}] -- User authentication failed: {}", "UserService::auth", e);
                            HttpResponse::Unauthorized().finish()
                        }
                    }
                },
                Err(e) => {
                    error!("[{}] -- User authentication failed: {}", "UserService::auth", err);
                    error!("[{}] -- User not found", "UserService::auth");
                    HttpResponse::Unauthorized().finish()
                }
            }
        }
    }
}


pub async fn get_user(_req: HttpRequest, info: web::Path<UserIdentifier>) -> HttpResponse {
    info!("[{}] -- Search user", "UserService::get_user");

    let user_id = match get_id_from_req(info) {
        Ok(x) => x,
        Err(e) => {
            error!("[{}] -- {}", "UserService::get_user", e);
            return HttpResponse::BadRequest().body(e);
        }
    };

    let user = database::get_user(Mode::Id(user_id)).await;
    match user {
        Ok(user) => {
            info!("[{}] -- Found user with id {}", "UserService::get_user", &user.id);

            let response = ResUser {
                id: user.id,
                name: user.name,
                email: user.email,
                created_at: user.created_at.to_string(),
                updated_at: user.updated_at.to_string(),
            };

            HttpResponse::Ok().json(serde_json::to_value(&response).unwrap())
            // return HttpResponse::Ok().json(serde_json::to_string_pretty(&user).unwrap());
        },
        Err(e) => {
            let body = match e {
                diesel::result::Error::DatabaseError(_kind, info) => {
                    warn!("[{}] -- {}", "UserService::get_user", info.message());
                    info.message().to_string()
                },
                _ => {
                    error!("[{}] -- {}", "UserService::get_user", e);
                    "".to_string()
                }
            };
            HttpResponse::NotFound().body(body)
        }
    }
}

pub async fn delete_user(_req: HttpRequest, info: web::Path<UserIdentifier>) -> HttpResponse {
    info!("[{}] -- Delete user", "User");

    let user_id = match get_id_from_req(info) {
        Ok(x) => x,
        Err(e) => {
            error!("[{}] -- {}", "UserService::get_user", e);
            return HttpResponse::BadRequest().body(e);
        }
    };

    HttpResponse::Ok().body(format!("User with id: {} deleted", user_id))
}

pub async fn list(_req: HttpRequest, user: Option<Identity>) -> HttpResponse {
    if user.is_none() {
        error!("[{}] -- Unauthorized", "UserService::list");
        return HttpResponse::Unauthorized().finish();
    }

    info!("[{}] -- Listing users..", "UserService::list");
    let users = database::get_users().await;

    match users {
        Ok(users) => {
            info!("[{}] -- Found {} users", "UserService::list", users.len());
            HttpResponse::Ok().json(serde_json::to_value(&users).unwrap())
        },
        Err(e) => {
            let body = match e {
                diesel::result::Error::DatabaseError(_kind, info) => {
                    warn!("[{}] -- {}", "UserService::list", info.message());
                    info.message().to_string()
                },
                _ => {
                    error!("[{}] -- {}", "UserService::list", e);
                    "".to_string()
                }
            };
            HttpResponse::NotFound().body(body)
        }
    }
}

pub async fn create(_data: web::Data<crate::AppState>, _req: HttpRequest, body: web::Json<CreateUser>) -> HttpResponse {
    info!("[{}] -- Creating user..", "User");

    // check
    if body.username.is_empty() {
        error!("[{}] -- No name provided", "UserService::create");
        return HttpResponse::BadRequest().body("No name provided");
    }
    if body.email.is_empty() {
        error!("[{}] -- No email provided", "UserService::create");
        return HttpResponse::BadRequest().body("No email provided");
    }
    if body.password.is_empty() {
        error!("[{}] -- No password provided", "UserService::create");
        return HttpResponse::BadRequest().body("No password provided");
    }

    let user_creation = web::block(move || {
        let password = generate_hash(body.password.as_str());

        database::create_user(&body.0, password)
    }).await;

    match user_creation {
        Ok(user) => {
            match user {
                Ok(_user) => {
                    HttpResponse::Ok().finish()
                },
                Err(e) => {
                    let body = match e {
                        diesel::result::Error::DatabaseError(_kind, info) => {
                            warn!("[{}] -- {}", "UserService::create", info.message());
                            warn!("[{}] -- {:?}", "UserService::create", info.details());
                            warn!("[{}] -- {:?}", "UserService::create", info.constraint_name());
                            
                            match info.constraint_name() {
                                Some("users_email_unique") => {
                                    "Email already used"
                                },
                                Some("users_name_unique") => {
                                    "Username already used"
                                },
                                _ => {
                                    "Internal server error"
                                }
                            }
                        },
                        _ => {
                            error!("[{}] -- {}", "UserService::create", e);
                            "Error"
                        }
                    };
                    HttpResponse::NotFound().body(body)
                }
            }
        },
        Err(e) => {
            warn!("[{}] -- Error: {}", "UserService::create", &e);
            HttpResponse::NotFound().body("User not created")
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use actix_web::{
        http::{self, header::ContentType},
        test,
    };

    #[actix_web::test]
    async fn test_index_ok() {
        let req = test::TestRequest::get()
            .insert_header(ContentType::plaintext())
            .to_http_request();
        let resp = list(req).await;
        assert_eq!(resp.status(), http::StatusCode::OK);
    }
    // #[actix_web::test]
    // async fn test_index_not_ok() {
    //     let req = test::TestRequest::default().to_http_request();
    //     let resp = index(req).await;
    //     assert_eq!(resp.status(), http::StatusCode::BAD_REQUEST);
    // }
}
