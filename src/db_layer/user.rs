use mysql::*;
use mysql::prelude::*;
use rocket::serde::json::Json;
use crate::URL;
use uuid::{uuid, Uuid};
use rocket::http::{CookieJar, Cookie};
use base64::{engine as _, engine::{self, general_purpose}, alphabet, Engine};
use rocket::form::validate::len;

use crate::{db_layer, services};

use services::user::{CreateUser, Login};

pub fn login(
    cookies: &CookieJar<'_>,
    credentials: Json<Login>
) -> bool {
    let mut conn = db_layer::connection::connect();

    let query = "SELECT user.id FROM user WHERE name = ? AND password = ?";
    let user_id:std::option::Option<u32> = conn.exec_first(query, (credentials.name.to_owned(), credentials.password.to_owned(), )).unwrap();

    if user_id.is_none() {
        return false
    }

    let uuid = Uuid::new_v4();
    let uuid = general_purpose::STANDARD.encode(uuid);

    println!("Session: {}", uuid);

    let query = "INSERT INTO user_auth_cookie (user_id, cookie) VALUES (?, ?) ON DUPLICATE KEY UPDATE cookie=?";
    let result = conn.exec_iter(query, (user_id.clone(), uuid.clone(), uuid.clone())).unwrap();

    if result.affected_rows() == 0 {
        return false
    }

    cookies.add(Cookie::new("session", uuid.to_string()));

    return true
}

pub fn get_id_by_session(
    cookie: String
) -> u32 {
    let mut conn = db_layer::connection::connect();

    println!("{}", cookie);

    let query = "SELECT user.id FROM user JOIN user_auth_cookie uac on user.id = uac.user_id WHERE uac.cookie = \"".to_owned()+&cookie.to_owned()+"\"";

    let result = conn.query(query).unwrap();

    println!("GetIdBySession: {:?}", result);
    
    if result.len() >= 1 {
        return result[0];
    }

    return 0;
}

pub fn logout(
    cookies: &CookieJar<'_>
) {
    cookies.remove(Cookie::named("session"))
}

pub fn is_admin(
    user_id: u32
) -> bool {
    let mut conn = db_layer::connection::connect();
    let query = "SELECT admin from user where id = ?";
    let result = conn.exec(query, (user_id, )).unwrap();
    println!("IsAdmin: {:?}", result);
    if result.is_empty() {
        return false;
    }

    return result[0];
}

pub fn create_account(
    account: Json<CreateUser>
) -> u64 {
    let mut conn = db_layer::connection::connect();
    let query = "SELECT id FROM user WHERE name = ?";
    let result: Vec<(u32)> = conn.exec(query, (account.name.clone(), )).unwrap();
    if result.is_empty() {
        let query = "INSERT INTO user (name, password) VALUES (?, ?)";
        let result = conn.exec_iter(query, (account.name.to_owned(), account.password.to_owned())).unwrap();
        return result.affected_rows()
    }
    return 0
}