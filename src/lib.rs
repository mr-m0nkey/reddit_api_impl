#![feature(proc_macro_hygiene, plugin, decl_macro)]
#[macro_use]
extern crate diesel;
extern crate dotenv;
#[macro_use]
extern crate rocket;
extern crate rocket_contrib;
#[macro_use]
extern crate serde_derive;


pub mod data;
pub mod routes;
pub mod schema;
pub mod service;


