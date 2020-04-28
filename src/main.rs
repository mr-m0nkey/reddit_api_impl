#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] 
extern crate rocket;
extern crate diesel;


extern crate reddit_api_impl;

use self::diesel::prelude::*;
use reddit_api_impl::data;



use diesel::mysql::MysqlConnection;

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

fn main() {

    let conn = data::establish_connection();


    rocket::ignite().mount("/", routes![index]).launch();
}


