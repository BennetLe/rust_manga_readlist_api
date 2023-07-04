use mysql::*;
use mysql::prelude::*;
use rocket::serde::json::{Json};
use rocket::serde::{Serialize, Deserialize};
use rocket::http::CookieJar;
use rocket::response::{Flash, Redirect};

use crate::db_layer;

#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Hash,Debug,Serialize,Deserialize, FromRow)]
#[serde(crate = "rocket::serde")]
pub struct Cookie{
    pub cookie: String
}

#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Hash,Debug,Serialize,Deserialize, FromRow)]
#[serde(crate = "rocket::serde")]
pub struct AllUserManga {
    pub id: u32,
    pub name: String,
    pub current_chapter: u32
}