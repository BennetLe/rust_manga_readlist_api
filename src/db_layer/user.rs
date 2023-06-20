use mysql::*;
use mysql::prelude::*;
use rocket::serde::json::Json;
use crate::URL;

use crate::{db_layer, services};

use services::user::{CreateUser, Login};

pub fn login(
    mut credentials: Json<Login>
) -> bool {
    let mut conn = db_layer::connection::connect();
    /// TODO: Get user id with matching credentials
    /// TODO: Change session token of the user id
    /// TODO: Add the cookie to the browser
    /// TODO: Return true and if no match false
}