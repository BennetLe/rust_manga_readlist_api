use mysql::*;
use mysql::prelude::*;
use rocket::serde::json::{Json};
use rocket::serde::{Serialize, Deserialize};
use rocket::http::CookieJar;
use rocket::response::{Flash, Redirect};

use crate::db_layer;

#[derive(Debug, Deserialize, Serialize)]
#[serde(crate = "rocket::serde")]
pub struct UserList {
    pub id: u32,
    pub name: String,
    pub private: bool,
    pub user_id: u32,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(crate = "rocket::serde")]
pub struct CreateUserList {
    pub name: String,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(crate = "rocket::serde")]
pub struct AddMangaToList {
    pub manga_id: u32,
    pub user_list_id: u32
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(crate = "rocket::serde")]
pub struct UpdateCurrentChapter {
    pub manga_list_id: u32,
    pub current_chapter: u32
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(crate = "rocket::serde")]
pub struct GetUserList {
    pub id: u32
}