use crate::data::bindings::UserForm;
use crate::data::repositories::user;
use rocket::request::Form;

#[get("/")]
pub fn all_users() -> &'static str {
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

#[post("/", data = "<user_form>")]
fn new(user_form: Form<UserForm>) {}
