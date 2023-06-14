use mysql::Pool;
use rocket::serde::json::{Json};
use rocket::serde::{Serialize, Deserialize};
use rocket::http::CookieJar;
use rocket::response::Redirect;

use crate::{services, db_layer};

#[get("/AllMangas")]
pub fn get_all_mangas() -> Json<Vec<(u32, String, u32, bool, bool)>> {
    Json(db_layer::manga::get_all())
}

#[post("/CreateMangaDemo")]
pub fn create_manga_demo() -> Json<u64> {
    Json(db_layer::manga::add())
}