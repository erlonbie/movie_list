-- Your SQL goes here-- Your SQL goes here
CREATE TABLE IF NOT EXISTS blog_posts (
  id SERIAL,
  title VARCHAR ,
  body TEXT NOT NULL,
  published BOOLEAN NOT NULL DEFAULT 'f',
  PRIMARY KEY (title)
);
