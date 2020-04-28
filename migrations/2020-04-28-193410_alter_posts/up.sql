ALTER TABLE posts
ADD CONSTRAINT FK_communiy_id_posts FOREIGN KEY (communiy_id) REFERENCES communities(id);