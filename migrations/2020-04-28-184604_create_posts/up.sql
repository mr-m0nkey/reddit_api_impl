CREATE TABLE IF NOT EXISTS posts
(
    id BIGINT NOT NULL PRIMARY KEY AUTO_INCREMENT,
    title VARCHAR(255) NOT NULL,
    body TEXT,
    author_id BIGINT NOT NULL,
    communiy_id BIGINT NOT NULL,
    upvotes BIGINT NOT NULL DEFAULT 0,
    downvotes BIGINT NOT NULL DEFAULT 0,
    is_edited TINYINT(1) NOT NULL DEFAULT 0,
    CONSTRAINT FK_author_id_posts FOREIGN KEY (author_id) REFERENCES users(id)
);


