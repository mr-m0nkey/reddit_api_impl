#[get("/")]
pub fn all_users() -> &'static str {
    "All users"
}

#[get("/a")]
pub fn get_user() -> &'static str {
    "get a user"
}
