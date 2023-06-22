use mysql::Pool;
use rocket::serde::json::{Json};
use rocket::serde::{Serialize, Deserialize};
use rocket::http::CookieJar;
use rocket::response::Redirect;

use crate::{services, db_layer};

use services::manga::CreateManga;

#[get("/Manga/all")]
pub fn get_all_mangas() -> Json<Vec<(u32, String, u32, bool, bool)>> {
    Json(db_layer::manga::get_all())
}

#[post("/Manga", format = "json", data="<manga>")]
pub fn create_manga(manga: Json<CreateManga>, cookie: &CookieJar<'_>) -> Json<u64> {
    if cookie.get("session").is_none() {
        return Json(0)
    };

    let cookie_session = cookie.get("session").unwrap().value();
    println!("Cookie: {:?}", cookie_session);
    let user_id = db_layer::user::get_id_by_session(cookie_session.to_string());
    let is_admin = db_layer::user::is_admin(user_id);

    if is_admin {
        return Json(db_layer::manga::add(manga))
    };

    return Json(0)
}