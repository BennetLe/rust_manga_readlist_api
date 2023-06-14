use mysql::*;
use mysql::prelude::*;
use crate::URL;

/// SOURCE: https://abackend.guru/rust/how-to-working-with-mysql-in-rust/

pub fn connect() -> PooledConn {
    let pool = Pool::new(URL).unwrap();
    let conn = pool.get_conn().unwrap();
    return conn;
}