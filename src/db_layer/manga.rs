use mysql::*;
use mysql::prelude::*;
use crate::URL;

use crate::db_layer;

#[derive(Debug, PartialEq, Eq)]
pub struct  Manga {
    pub name: String,
    pub chapters: u32,
}

pub fn get_all() -> Vec<(u32, String, u32, bool, bool)> {
    let mut conn = db_layer::connection::connect();
    let query = "SELECT * FROM mangas";
    let result = conn.query(query).unwrap();
    return result;
}

pub fn add() -> u64 {
    let mut conn = db_layer::connection::connect();
    let query = "INSERT INTO mangas (name, chapters) VALUES (?, ?)";
    let name = "XD";
    let chapters:u32= 187;
    let result = conn.exec_iter(query, (name, chapters)).unwrap();
    return result.affected_rows();
}