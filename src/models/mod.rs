

#[derive(Debug)]
pub struct Post {
    title: String,
    body: Option<String>,
    is_edited: bool,
    upvotes: i32,
    downvotes: i32
}


#[derive(Debug)]
pub struct Comment {
    text: String,
    is_edited: bool,
    replies: Vec<Comment>,
    upvotes: i32,
    downvotes: i32
}

#[derive(Debug)]
pub struct User {
    username: String,
    password: String
}

// implementations
impl Post {

    pub fn new(title: &str, body: Option<String>) -> Post {
        Post {
            title: title.to_string(),
            body: body,
            is_edited: false,
            upvotes: 0,
            downvotes: 0
        }
    }

}

impl Comment {

    pub fn new(text: &str) -> Comment {
        Comment {
            text: text.to_string(),
            is_edited: false,
            replies: Vec::new(),
            upvotes: 0,
            downvotes: 0
        }
    }

}

impl User {
    
    pub fn new(username: &str, password: &str) -> User {
        User {
            username: username.to_string(),
            password: password.to_string()
        }
    }

}
