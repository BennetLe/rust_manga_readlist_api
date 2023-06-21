use mysql::*;
use mysql::prelude::*;
use rocket::serde::json::Json;
use crate::URL;
use uuid::{uuid, Uuid};
use rocket::http::{CookieJar, Cookie};

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
    let query = "INSERT INTO user_auth_cookie (user_id, cookie) VALUES (?, ?) ON DUPLICATE KEY UPDATE cookie=?";
    let result = conn.exec_iter(query, (user_id.clone(), uuid, uuid)).unwrap();

    if result.affected_rows() == 0 {
        return false
    }

    cookies.add(Cookie::new("session", uuid.to_string()));

    return true
}

pub fn get_id_by_session(
    cookie: String
) -> Vec<(u32)> {
    let mut conn = db_layer::connection::connect();
    let query = "SELECT user.id FROM user JOIN user_auth_cookie uac on user.id = uac.user_id WHERE uac.cookie = ?";
    let result = conn.exec(query, (cookie, )).unwrap();
    return result;
}