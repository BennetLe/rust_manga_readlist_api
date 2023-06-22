use mysql::*;
use mysql::prelude::*;
use rocket::http::CookieJar;
use rocket::serde::json::Json;
use crate::URL;

use crate::{db_layer, services};

use services::user_list::CreateUserList;
use crate::services::user_list::AddMangaToList;

pub fn get_all() -> Vec<(u32, String, bool, u32)> {
    let mut conn = db_layer::connection::connect();
    let query = "SELECT * FROM user_list";
    let result = conn.query(query).unwrap();
    return result;
}

pub fn create(
    new_user_list: Json<CreateUserList>,
    cookie: &CookieJar<'_>
) -> u64 {
    let name = new_user_list.name.to_owned();
    let mut conn = db_layer::connection::connect();
    let query = "SELECT id FROM user_list WHERE name = ?";
    let result :Vec<u32> = conn.exec(query, (name.clone(), )).unwrap();

    if result.is_empty() {
        return 0;
    }


    let cookie_session = cookie.get("session").unwrap().value();
    let user_id = db_layer::user::get_id_by_session(cookie_session.to_string());

    if user_id >= 1 {
        let mut conn = db_layer::connection::connect();
        let query = "INSERT INTO user_list (name, user_id) VALUES (?, ?)";
        let result = conn.exec_iter(query, (name, user_id)).unwrap();
        return result.affected_rows();
    };

    return 0;
}

pub fn add_maga_to_list(
    manga_list: Json<AddMangaToList>
) -> u8 {


    return 0;
}