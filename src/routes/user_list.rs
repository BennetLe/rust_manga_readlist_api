use mysql::Pool;
use rocket::serde::json::{Json};
use rocket::serde::{Serialize, Deserialize};
use rocket::http::CookieJar;
use rocket::response::Redirect;

use crate::{services, db_layer};

use services::user_list::CreateUserList;

#[get("/UserList/all")]
pub fn get_all_user_lists() -> Json<Vec<(u32, String, bool, u32)>> {
    Json(db_layer::user_list::get_all())
}

#[post("/UserList", format="json", data="<user_list>")]
pub fn create_user_list(cookies: &CookieJar<'_>, user_list: Json<CreateUserList>) -> Json<u64> {
    Json(db_layer::user_list::create(user_list, cookies))
}