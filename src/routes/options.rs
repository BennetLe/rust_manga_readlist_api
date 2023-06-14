use rocket::serde::json::{Json};
use rocket::serde::{Serialize, Deserialize};
use rocket::http::CookieJar;
use rocket::response::{Flash, Redirect};

#[options("/<_..>")]
pub fn all_options() {
    /* Intentionally left empty */
}