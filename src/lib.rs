#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use]
extern crate rocket;

#[macro_use]
extern crate diesel;
extern crate dotenv;

pub mod data;
pub mod routes;
pub mod schema;
