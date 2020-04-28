#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use]
extern crate rocket;
extern crate reddit_api_impl;

use diesel::mysql::MysqlConnection;
use reddit_api_impl::data;

fn main() {
    let conn = data::establish_connection();

    rocket::ignite().mount("/", routes![index]).launch();
}

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}
