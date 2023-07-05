use mysql::Pool;
use rocket::serde::json::{Json};
use rocket::serde::{Serialize, Deserialize};
use rocket::http::CookieJar;
use rocket::response::Redirect;

use crate::{services, db_layer};

use services::user_list::CreateUserList;
use crate::services::user_list::{AddMangaToList, UpdateCurrentChapter};

#[get("/UserList/all")]
pub fn get_all_user_lists(cookies: &CookieJar<'_>) -> Json<Vec<(services::user_list::UserList)>> {
    Json(db_layer::user_list::get_all(cookies))
}

#[get("/UserList", format="json", data="<list_id>")]
pub fn get_mangas_from_list(cookies: &CookieJar<'_>, list_id: Json<services::user_list::GetUserList>) -> Json<Vec<(services::user_list::UserList)>> {
    Json(db_layer::user_list::get(cookies, list_id.id))
}

#[post("/UserList", format="json", data="<user_list>")]
pub fn create_user_list(cookies: &CookieJar<'_>, user_list: Json<CreateUserList>) -> Json<services::logic::Success> {
    Json(services::logic::Success{
        success: if db_layer::user_list::create(user_list, cookies)==1 {true} else { false }
    })
}

#[post("/UserList/add", format="json", data="<manga_list>")]
pub fn add_manga_to_list(cookies: &CookieJar<'_>, manga_list: Json<AddMangaToList>) -> Json<services::logic::Success> {
    return Json(services::logic::Success{
        success: if db_layer::user_list::add_maga_to_list(cookies, manga_list)==1 {true} else { false }
    })
}

#[post("/UserList/update", format="json", data="<update_chapter>")]
pub fn update_current_chapter(cookies: &CookieJar<'_>, update_chapter: Json<UpdateCurrentChapter>) -> Json<services::logic::Success> {
    return Json(services::logic::Success{
        success: if db_layer::user_list::change_current_chapter(cookies, update_chapter)==1 {true} else { false }
    })
}