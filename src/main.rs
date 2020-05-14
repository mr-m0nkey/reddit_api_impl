#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use]
extern crate rocket;
extern crate reddit_api_impl;

use diesel::mysql::MysqlConnection;
use reddit_api_impl::data;

fn main() {
    let conn = data::establish_connection();

    rocket::ignite()
        .mount("/", routes![welcome])
        .mount(
            "/users",
            routes![
                reddit_api_impl::routes::user::get_all_users,
                reddit_api_impl::routes::user::get_user,
                reddit_api_impl::routes::user::delete_user,
                reddit_api_impl::routes::user::add_user,
                reddit_api_impl::routes::user::update_user
            ],
        )
        .launch();
}

#[get("/")]
fn welcome() -> &'static str {
    "Welcome"
}
