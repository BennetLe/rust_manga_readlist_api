use mysql::*;
use mysql::prelude::*;
use rocket::serde::json::Json;
use crate::URL;

use crate::{db_layer, services};

pub fn get_all_user_mangas(
    mut cookie_json: Json<services::logic::Cookie>
) -> Vec<(u32, String, String, u32)> {
    let mut conn = db_layer::connection::connect();
    let query = "SELECT ml.id, user.name as user, m.name as manga, ml.current_chapter as chapter
FROM user
    JOIN user_list ul on user.id = ul.user_id
    JOIN manga_list ml on ul.id = ml.user_list_id
    JOIN mangas m on m.id = ml.manga_id
    JOIN user_auth_cookie uac on user.id = uac.user_id
where uac.cookie = ?";
    let cookie = cookie_json.cookie.to_owned();
    let result = conn.exec(query, (cookie, )).unwrap();
    return result;
}