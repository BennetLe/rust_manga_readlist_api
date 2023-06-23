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
        let cookie_session = cookie.get("session").unwrap().value();
        let user_id = db_layer::user::get_id_by_session(cookie_session.to_string());

        if user_id >= 1 {
            let mut conn = db_layer::connection::connect();
            let query = "INSERT INTO user_list (name, user_id) VALUES (?, ?)";
            let result = conn.exec_iter(query, (name, user_id)).unwrap();
            return result.affected_rows();
        };
    }

    return 0;
}

pub fn add_maga_to_list(
    cookie: &CookieJar<'_>,
    manga_list: Json<AddMangaToList>
) -> u64 {
    let mut conn = db_layer::connection::connect();
    let result = is_owner_of_list(cookie, manga_list.user_list_id.clone());

    if result {
        let query = "INSERT INTO manga_list (manga_id, user_list_id, current_chapter) VALUES (?, ?, 0)";
        let result = conn.exec_iter(query, (manga_list.manga_id, manga_list.user_list_id)).unwrap();

        return result.affected_rows();
    }

    return 0;
}

pub fn is_owner_of_list(
    cookie: &CookieJar<'_>,
    user_list_id: u32
) -> bool{
    let mut conn = db_layer::connection::connect();
    let cookie_session = cookie.get("session").unwrap().value();
    let user_id = db_layer::user::get_id_by_session(cookie_session.to_string());
    let query = "SELECT user_id FROM user_list WHERE user_id = ? AND id = ?";
    let result: Vec<u32> = conn.exec(query, (user_id, user_list_id)).unwrap();

    if result.len() == 0 {
        return false
    }
    if result[0] >= 1 {
        return true
    }
    return false
}