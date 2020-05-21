use crate::schema::users;

#[derive(Queryable, AsChangeset, Serialize, Deserialize)]
pub struct User {
    pub id: i64,
    pub username: String,
    pub password: String,
}


#[derive(Queryable)]
pub struct Post {
    id: i64,
    title: String,
    body: Option<String>,
    author_id: i64,
    community_id: i64,
    upvotes: i64,
    downvotes: i64,
    is_edited: bool,
}

#[derive(Queryable)]
pub struct Comment {
    id: i64,
    body: String,
    author_id: i64,
    parent_comment_id: Option<i64>,
    post_id: i64,
    upvotes: i64,
    downvotes: i64,
    is_edited: bool,
}

#[derive(Queryable)]
pub struct Community {
    id: i64,
    name: String,
    description: String,
    creator_id: i64,
}
