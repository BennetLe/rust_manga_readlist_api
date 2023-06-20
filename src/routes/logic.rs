use mysql::Pool;
use rocket::serde::json::{Json};
use rocket::serde::{Serialize, Deserialize};
use rocket::http::CookieJar;
use rocket::response::Redirect;

use crate::{services, db_layer};

#[post("/MangaList/all")]
pub fn get_all_user_mangas(cookies: &CookieJar<'_>) -> Json<Vec<(u32, String, String, u32)>> {
    let cookie_json = Json(
        services::logic::Cookie {
            cookie: cookies.get("session").map(|crumb| crumb.value()).unwrap().to_string()
        }
    );
    Json(db_layer::logic::get_all_user_mangas(cookie_json))
}