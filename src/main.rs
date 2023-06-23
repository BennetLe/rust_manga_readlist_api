#[macro_use] extern crate rocket;

use mysql::{Pool, PooledConn};
use crate::routes::options::all_options;
use crate::routes::manga::{get_all_mangas, create_manga};
use crate::routes::user_list::{get_all_user_lists, create_user_list, add_manga_to_list, update_current_chapter, get_mangas_from_list};
use crate::routes::logic::{get_all_user_mangas};
use crate::routes::user::{login, logout};

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
        .mount("/api", routes![
                all_options,
                get_all_mangas,
                create_manga,
                get_all_user_lists,
                create_user_list,
                get_all_user_mangas,
                login,logout,
                add_manga_to_list,
                update_current_chapter,
                get_mangas_from_list,
        ])
}

// TODO: Add an Endpoint to see all list + mangas in a specific list
// TODO: Add an Endpoint for Admins to change the chapter count of a Manga
// TODO: Add an Endpoint to create an Account