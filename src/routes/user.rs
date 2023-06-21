use mysql::Pool;
use rocket::serde::json::{Json};
use rocket::serde::{Serialize, Deserialize};
use rocket::http::CookieJar;
use rocket::response::Redirect;

use crate::{services, db_layer};

#[post("/User/login", format = "json", data="<credentials>")]
pub fn login(cookies: &CookieJar<'_>, credentials: Json<services::user::Login>) -> Json<services::user::LoginResult> {
    let success = services::user::LoginResult{
        success: db_layer::user::login(cookies, credentials)
    };
    return Json(success)
}