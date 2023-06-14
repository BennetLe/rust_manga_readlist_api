use mysql::*;
use mysql::prelude::*;
use rocket::serde::json::Json;
use crate::URL;

use crate::{db_layer, services};

use services::user_list::CreateUserList;

pub fn get_all() -> Vec<(u32, String, bool, u32)> {
    let mut conn = db_layer::connection::connect();
    let query = "SELECT * FROM user_lists";
    let result = conn.query(query).unwrap();
    return result;
}

