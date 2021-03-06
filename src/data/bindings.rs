use super::models::*;
use crate::schema::users;

#[derive(Insertable)]
#[table_name = "users"]
pub struct NewUser {
    pub username: String,
    pub password: String,
}

impl NewUser {
    fn from_user(user: User) -> NewUser {
        NewUser {
            username: user.username,
            password: user.password,
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct UserForm {
    pub username: String,
    pub password: Option<String>,
}

impl UserForm {
    pub fn update_user(self, user: &mut User) {
        user.username = self.username;
    }

    pub fn to_new_user(self) -> NewUser {
        //TODO return Result<NewUser> instead

        if let Some(passwword) = self.password {
            NewUser {
                username: self.username,
                password: passwword,
            }
        } else {
            panic!("A password is required");
        }
    }
}

pub struct ResponseDto<T> {
    pub response_code: String,
    pub response_message: String,
    pub data: Option<T>
}
