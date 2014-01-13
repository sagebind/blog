DROP TABLE IF EXISTS blog_post;
CREATE TABLE blog_post (
    id          INT             NOT NULL AUTO_INCREMENT PRIMARY KEY,
    created_at  DATETIME        NOT NULL,
    updated_at  DATETIME        NOT NULL,
    title       VARCHAR(255)    NOT NULL,
    slug        VARCHAR(255)    NOT NULL,
    content     TEXT            NOT NULL,
    UNIQUE KEY (slug)
);

DROP TABLE IF EXISTS image;
CREATE TABLE image (
    id          INT             NOT NULL AUTO_INCREMENT PRIMARY KEY,
    created_at  DATETIME        NOT NULL,
    updated_at  DATETIME        NOT NULL,
    description VARCHAR(255)    NOT NULL,
    data        BLOB            NOT NULL
);
