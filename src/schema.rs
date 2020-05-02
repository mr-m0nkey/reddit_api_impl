table! {
    comments (id) {
        id -> Bigint,
        body -> Text,
        author_id -> Bigint,
        parent_comment_id -> Nullable<Bigint>,
        post_id -> Bigint,
        upvotes -> Bigint,
        downvotes -> Bigint,
        is_edited -> Bool,
    }
}

table! {
    communities (id) {
        id -> Bigint,
        name -> Text,
        description -> Text,
        creator_id -> Bigint,
    }
}

table! {
    posts (id) {
        id -> Bigint,
        title -> Varchar,
        body -> Nullable<Text>,
        author_id -> Bigint,
        communiy_id -> Bigint,
        upvotes -> Bigint,
        downvotes -> Bigint,
        is_edited -> Bool,
    }
}

table! {
    users (id) {
        id -> Bigint,
        username -> Varchar,
        password -> Varchar,
    }
}

joinable!(comments -> posts (post_id));
joinable!(comments -> users (author_id));
joinable!(communities -> users (creator_id));
joinable!(posts -> communities (communiy_id));
joinable!(posts -> users (author_id));

allow_tables_to_appear_in_same_query!(
    comments,
    communities,
    posts,
    users,
);
