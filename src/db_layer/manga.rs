use mysql::*;
use mysql::prelude::*;
use rocket::serde::json::Json;
use crate::URL;

use crate::{db_layer, services};

use services::manga::CreateManga;

pub fn get_all() -> Vec<(u32, String, u32, bool, bool)> {
    let mut conn = db_layer::connection::connect();
    let query = "SELECT * FROM mangas";
    let result = conn.query(query).unwrap();
    return result;
}

pub fn add(
    mut new_manga: Json<CreateManga>
) -> u64 {
    let mut conn = db_layer::connection::connect();
    let query = "INSERT INTO mangas (name, chapters) VALUES (?, ?)";
    let name = new_manga.name.to_owned();
    let chapters:u32= new_manga.chapters;
    let result = conn.exec_iter(query, (name, chapters)).unwrap();
    return result.affected_rows();
}