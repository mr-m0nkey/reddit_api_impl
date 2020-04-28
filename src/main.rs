#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] 
extern crate rocket;
#[macro_use] 
extern crate diesel;
#[macro_use] 
extern crate dotenv;


extern crate reddit_api_impl;

use self::diesel::prelude::*;
use reddit_api_impl::data::models::*;



use diesel::prelude::*;
use diesel::mysql::MysqlConnection;
use dotenv::dotenv;
use std::env;

pub fn establish_connection() -> MysqlConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");
        MysqlConnection::establish(&database_url)
        .expect(&format!("Error connecting to {}", database_url))
}


#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

fn main() {

    use reddit_api_impl::schema::communities::dsl::*;


    let connection = establish_connection();
    let results = communities
        .limit(5)
        .load::<Community>(&connection)
        .expect("Error loading posts");



    rocket::ignite().mount("/", routes![index]).launch();
}