use mysql::Pool;
use rocket::serde::json::{Json};
use rocket::serde::{Serialize, Deserialize};
use rocket::http::CookieJar;
use rocket::response::Redirect;

use crate::{services, db_layer};

#[get("/MangaList/all")]
pub fn get_all_user_mangas(cookies: &CookieJar<'_>) -> Json<Vec<(services::logic::AllUserManga)>> {
    if cookies.get("session").is_none() {
        return Json(Vec::new())
    }
    Json(db_layer::logic::get_all_user_mangas(cookies.get("session").unwrap().value().to_string()))
}