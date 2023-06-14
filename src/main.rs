#[macro_use] extern crate rocket;

use mysql::{Pool, PooledConn};
use crate::routes::manga::get_all_mangas;
use crate::routes::options::all_options;
use crate::routes::manga::create_manga_demo;

mod cors;
mod db_layer;
mod routes;
mod services;

pub struct CORS;

pub static URL: &str = "mysql://root@localhost:3306/rust_manga_readlist";

#[launch]
fn rocket() -> _ {
    rocket::build()
        .attach(cors::Cors)
        .mount("/api", routes![all_options, get_all_mangas, create_manga_demo])
}