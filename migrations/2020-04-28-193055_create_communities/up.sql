CREATE TABLE IF NOT EXISTS communities
(
    id BIGINT NOT NULL PRIMARY KEY AUTO_INCREMENT,
    name TEXT NOT NULL,
    description TEXT NOT NULL,
    creator_id BIGINT NOT NULL,
    CONSTRAINT FK_creator_id_communities FOREIGN KEY (creator_id) REFERENCES users(id)
);


