use mysql::*;
use mysql::prelude::*;
use rocket::serde::json::Json;
use crate::URL;

use crate::{db_layer, services};

use services::user_list::CreateUserList;

pub fn get_all() -> Vec<(u32, String, bool, u32)> {
    let mut conn = db_layer::connection::connect();
    let query = "SELECT * FROM user_list";
    let result = conn.query(query).unwrap();
    return result;
}

pub fn create(
    mut new_user_list: Json<CreateUserList>
) -> u64 {
    let mut conn = db_layer::connection::connect();
    let query = "INSERT INTO user_list (name, user_id) VALUES (?, ?)";
    let name = new_user_list.name.to_owned();
    let user_id = new_user_list.user_id;
    let result = conn.exec_iter(query, (name, user_id)).unwrap();
    return result.affected_rows();
}