use crate::data::bindings::UserForm;
use crate::data::bindings::ResponseDto;
use crate::data::models::User;
use crate::data::repositories::user as user_repo;
use std::error::Error;
use crate::data;




pub fn add_user(user_form: UserForm, connection: data::DbConn) -> Result<(), Box<dyn Error>> {

    let new_user = user_form.to_new_user();
    
    let result = user_repo::create_user(new_user, &connection);


    match result {
        Ok(User) => {
            Ok(())
        },
        Err(error) => {
            Err(Box::new(error))
        }
    }

}

pub fn get_users(connection: data::DbConn) -> ResponseDto<Vec<User>> {

   unimplemented!()



        
}