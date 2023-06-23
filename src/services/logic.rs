use mysql::*;
use mysql::prelude::*;
use rocket::serde::json::{Json};
use rocket::serde::{Serialize, Deserialize};
use rocket::http::CookieJar;
use rocket::response::{Flash, Redirect};

use crate::db_layer;

#[derive(Debug, Deserialize, Serialize)]
#[serde(crate = "rocket::serde")]
pub struct Cookie{
    pub cookie: String
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(crate = "rocket::serde")]
pub struct AllUserManga {
    pub manga_list_id: u32,
    pub user_name: String,
    pub manga_name: String,
    pub manga_list_chapter: u32
}