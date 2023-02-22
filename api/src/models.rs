use rocket::serde::{Deserialize, Serialize};

use diesel::{AsChangeset, Insertable, Queryable};

use crate::schema::blog_posts;

#[derive(Serialize, Deserialize, Queryable, AsChangeset, Debug, Insertable, Eq)]
#[table_name = "blog_posts"]
pub struct Movie {
    pub id: i32,
    pub title: String,
    pub body: String,
    pub published: bool,
}

impl PartialEq for Movie {
    fn eq(&self, other: &Self) -> bool {
        self.title == other.title
    }
}

impl Ord for Movie {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.title.cmp(&other.title)
    }
}

impl PartialOrd for Movie {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(Serialize, Deserialize, Queryable, AsChangeset, Debug, Insertable)]
#[table_name = "blog_posts"]
pub struct NewMovie {
    pub title: String,
    pub body: String,
    pub published: bool,
}
