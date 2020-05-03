use bindings::NewUser;
use models::User;

pub fn create_user(new_user: NewUser, conn: &MysqlConnection) -> User {
    use crate::schema::users;

    //TODO encrypt password

    diesel::insert_into(users::table)
        .values(&new_user)
        .execute(conn);

    users::table.order(users::id.desc()).first(conn).unwrap()
}


// edit 
// delete
// retrieve