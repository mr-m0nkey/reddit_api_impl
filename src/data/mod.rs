pub mod models;
pub mod bindings;

use models::*;
use bindings::*;
use crate::diesel::prelude::*;
use crate::diesel::mysql::MysqlConnection;
use dotenv::dotenv;
use std::env;



pub fn establish_connection() -> MysqlConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");
        MysqlConnection::establish(&database_url)
        .expect(&format!("Error connecting to {}", database_url))
}


pub fn create_user(conn: &MysqlConnection) -> User{
    use crate::schema::users;

    let new_user = NewUser {
        username: "efee".to_string(),
        password: "body".to_string(),
    };

    diesel::insert_into(users::table)
        .values(&new_user)
        .execute(conn);

    users::table.order(users::id.desc()).first(conn).unwrap() 
}

