-- Your SQL goes here
CREATE TABLE posts (
    id INT(11) NOT NULL PRIMARY KEY,
    title NVARCHAR NOT NULL,
    image TEXT NOT NULl,
    body TEXT NOT NULl,
    summary TEXT NOT NULl,
    published BOOLEAN NOT NULL,
    published_date TIMESTAMP NOT NULL
);