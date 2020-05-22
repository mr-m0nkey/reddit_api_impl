use crate::data::bindings::UserForm;
use crate::data::repositories::user;
use rocket_contrib::json::Json;


use crate::data;





#[get("/")]
pub fn get_all_users() -> &'static str {
    "All users"
}

#[get("/<id>")]
pub fn get_user(id: i64) -> &'static str {
    "get a user"
}

#[delete("/<id>")]
pub fn delete_user(id: i64) -> &'static str {
    "delete a user"
}

#[post("/", format = "application/json", data = "<user_form>")]
pub fn add_user(user_form: Json<UserForm>, connection: data::DbConn) {

    let new_user = user_form
        .into_inner()
        .to_new_user();
    
    user::create_user(new_user, &connection);

        
}

#[put("/<id>", format = "application/json", data = "<user_form>")]
pub fn update_user(id: i64, user_form: Json<UserForm>) {}



