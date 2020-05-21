#![feature(proc_macro_hygiene, plugin, decl_macro)]
extern crate diesel;
extern crate dotenv;
#[macro_use]
extern crate rocket;
extern crate rocket_contrib;
extern crate serde_derive;
extern crate reddit_api_impl;


use reddit_api_impl::data;

fn main() {

    rocket::ignite()
        .mount("/", routes![welcome])
        .manage(data::init_pool())
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
