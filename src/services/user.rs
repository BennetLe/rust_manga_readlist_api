use mysql::*;
use mysql::prelude::*;
use rocket::serde::json::{Json};
use rocket::serde::{Serialize, Deserialize};
use rocket::http::CookieJar;
use rocket::response::{Flash, Redirect};

use crate::db_layer;

#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Hash,Debug,Serialize,Deserialize, FromRow)]
#[serde(crate = "rocket::serde")]
pub struct User {
    pub id: u32,
    pub name: String,
    pub password: String,
    pub admin: bool
}

#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Hash,Debug,Serialize,Deserialize, FromRow)]
#[serde(crate = "rocket::serde")]
pub struct CreateUser {
    pub name: String,
    pub password: String
}

#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Hash,Debug,Serialize,Deserialize, FromRow)]
#[serde(crate = "rocket::serde")]
pub struct Login{
    pub name: String,
    pub password: String
}

#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Hash,Debug,Serialize,Deserialize, FromRow)]
#[serde(crate = "rocket::serde")]
pub struct LoginResult{
    pub success: bool
}

