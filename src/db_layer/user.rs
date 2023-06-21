use mysql::*;
use mysql::prelude::*;
use rocket::serde::json::Json;
use crate::URL;

use crate::{db_layer, services};

use services::user::{CreateUser, Login};

pub fn login(
    credentials: Json<Login>
) -> bool {
    let mut conn = db_layer::connection::connect();
    // TODO: Get user id with matching credentials
    // TODO: Change session token of the user id
    // TODO: Add the cookie to the browser
    // TODO: Return true and if no match false
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