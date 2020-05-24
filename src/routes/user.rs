use crate::data::bindings::UserForm;

use crate::service::user as user_service;
use rocket_contrib::json::Json;
use std::error::Error;
use crate::data;





#[get("/")]
pub fn get_all_users() -> &'static str {
    unimplemented!()

}

#[get("/<id>")]
pub fn get_user(id: i64) -> &'static str {
    unimplemented!()

}

#[delete("/<id>")]
pub fn delete_user(id: i64) -> &'static str {
    unimplemented!()

}

#[post("/", format = "application/json", data = "<user_form>")]
pub fn add_user(user_form: Json<UserForm>, connection: data::DbConn) {

    let result: Result<(), Box<dyn Error>> = user_service::add_user(user_form.into_inner(), connection);

    unimplemented!()



        
}

#[put("/<id>", format = "application/json", data = "<user_form>")]
pub fn update_user(id: i64, user_form: Json<UserForm>) {
    unimplemented!()

}



