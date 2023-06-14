use rocket::serde::json::{Json};
use rocket::http::{Cookie, CookieJar};
use rocket::response::Redirect;
use mysql::*;
use mysql::prelude::*;

use crate::db_layer;
