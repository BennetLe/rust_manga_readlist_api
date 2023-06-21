use mysql::Pool;
use rocket::serde::json::{Json};
use rocket::serde::{Serialize, Deserialize};
use rocket::http::CookieJar;
use rocket::response::Redirect;

use crate::{services, db_layer};

#[post("/User/login", format = "json", data="<login>")]
pub fn login(cookies: &CookieJar<'_>, login: Json<services::user::Login>) {

}