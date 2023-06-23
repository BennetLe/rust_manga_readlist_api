use mysql::*;
use mysql::prelude::*;
use rocket::serde::json::{Json};
use rocket::serde::{Serialize, Deserialize};
use rocket::http::CookieJar;
use rocket::response::{Flash, Redirect};

use crate::db_layer;

#[derive(Debug, Deserialize, Serialize)]
#[serde(crate = "rocket::serde")]
pub struct Manga {
    pub id: u32,
    pub name: String,
    pub chapters: u32,
    pub finished: bool,
    pub no_updates: bool,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(crate = "rocket::serde")]
pub struct CreateManga {
    pub name: String,
    pub chapters: u32,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(crate = "rocket::serde")]
pub struct UpdateChapters {
    pub chapter: u32,
    pub id: u32
}