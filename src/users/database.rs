use log::{error, warn, info, debug, trace, LevelFilter};
use diesel::{QueryDsl, RunQueryDsl};
use diesel::ExpressionMethods;
// use dotenv::dotenv;

use crate::database::{
    POOL, QueryResult
};
use crate::models::{
    User
};

use super::{Mode, CreateUser};

macro_rules! getConn {
    () => {
        &mut POOL.get().unwrap()
    };
}


fn find_user_by_name(_username: &String) -> QueryResult<User> {
    use crate::schema::users::dsl::*;
    let conn = getConn!();

    users
        .filter(name.eq(_username))
        .first::<User>(conn)
}

fn find_user_by_email(_email: &String) -> QueryResult<User> {
    use crate::schema::users::dsl::*;
    let conn = getConn!();

    users
        .filter(email.eq(_email))
        .first::<User>(conn)
}

fn find_user_by_id(user_id: &i32) -> QueryResult<User> {
    use crate::schema::users::dsl::*;
    let conn = getConn!();

    users.find(user_id).first(conn)
}

pub async fn get_user(user: Mode) -> Result<User, diesel::result::Error> {
    match user {
        Mode::Id(x) => {
            info!("[{}] -- Search user using id: {}", "UserService::get_user", &x);
            Ok(find_user_by_id(&x)?)
        },
        Mode::Username(x) => {
            info!("[{}] -- Search user using username: {}", "UserService::get_user", &x);
            Ok(find_user_by_name(&x)?)
        },
        Mode::Email(x) => {
            info!("[{}] -- Search user using email: {}", "UserService::get_user", &x);
            Ok(find_user_by_email(&x)?)
        },
    }
}

pub fn create_user(body: &CreateUser, pwd: String) -> Result<(), diesel::result::Error> {
    use crate::schema::users::dsl::*;
    let conn = getConn!();
    let _rows_inserted = diesel::insert_into(users)
        .values((
            name.eq(&body.username),
            email.eq(&body.email),
            password.eq(pwd),
            // created_at.eq(diesel::dsl::now),
            // updated_at.eq(diesel::dsl::now),
        ))
        .execute(conn)?;

    info!("[{}] -- Created user with email {}", "UserService::create_user", body.email);

    Ok(())
}

pub async fn get_users() -> Result<Vec<User>, diesel::result::Error> {
    use crate::schema::users::dsl::*;
    let conn = getConn!();
    let list = users.load::<User>(conn)?;
    Ok(list)
}

#[test]
fn test_find_user() {
    // dotenv().ok();
    // let username = "valentin".to_string();
    // let user = find_user_by_name(&username);
    // println!("host => {}", crate::HOST);
    println!("ratio => Host: {} Port {}", *crate::HOST, *crate::PORT);
}
