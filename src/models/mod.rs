

#[derive(Debug)]
pub struct Post {
    title: String,
    body: String,
    is_edited: bool
}


#[derive(Debug)]
pub struct Comment {
    text: String,
    is_edited: bool,
    replies: Vec<Comment>
}

#[derive(Debug)]
pub struct User {
    username: String,
    password: String
}

