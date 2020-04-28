CREATE TABLE IF NOT EXISTS comments
(
    id BIGINT NOT NULL PRIMARY KEY AUTO_INCREMENT,
    body TEXT NOT NULL,
    author_id BIGINT NOT NULL,
    parent_comment_id BIGINT,
    post_id BIGINT NOT NULL,
    upvotes BIGINT NOT NULL DEFAULT 0,
    downvotes BIGINT NOT NULL DEFAULT 0,
    is_edited TINYINT(1) NOT NULL DEFAULT 0,
    CONSTRAINT FK_author_id_comments FOREIGN KEY (author_id) REFERENCES users(id),
    CONSTRAINT FK_post_id_comments FOREIGN KEY (post_id) REFERENCES posts(id),
    CONSTRAINT FK_parent_comment_id_comments FOREIGN KEY (parent_comment_id) REFERENCES comments(id)
);


