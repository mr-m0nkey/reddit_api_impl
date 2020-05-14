use crate::data::bindings::NewUser;
use crate::data::models::User;
use crate::diesel;
use crate::diesel::mysql::MysqlConnection;
use crate::diesel::prelude::*;
use crate::schema::users;

pub fn create_user(new_user: NewUser, connection: &MysqlConnection) -> QueryResult<User> {
    //TODO encrypt password

    diesel::insert_into(users::table)
        .values(&new_user)
        .execute(connection);

    users::table.order(users::id.desc()).first(connection)
}

pub fn all(connection: &MysqlConnection) -> QueryResult<Vec<User>> {
    users::table.load::<User>(&*connection)
}

pub fn get(id: i64, connection: &MysqlConnection) -> QueryResult<User> {
    users::table.find(id).first(connection)
}

pub fn update(id: i64, user: User, connection: &MysqlConnection) -> QueryResult<usize> {
    diesel::update(users::table.find(id))
        .set(&user)
        .execute(connection)
}

pub fn delete(id: i64, connection: &MysqlConnection) -> QueryResult<usize> {
    diesel::delete(users::table.find(id)).execute(connection)
}
