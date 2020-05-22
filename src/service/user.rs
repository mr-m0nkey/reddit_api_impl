use crate::data::bindings::UserForm;
use crate::data::bindings::ResponseDto;
use crate::data::models::User;
use crate::data::repositories::user as user_repo;


use crate::data;




pub fn add_user(user_form: UserForm, connection: data::DbConn) -> ResponseDto<User> {

    let new_user = user_form.to_new_user();
    
    user_repo::create_user(new_user, &connection);


    unimplemented!()


        
}

pub fn get_users(connection: data::DbConn) -> ResponseDto<Vec<User>> {

   unimplemented!()



        
}