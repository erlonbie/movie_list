use diesel::prelude::*;
use rocket::{
response::status::{NoContent, NotFound},
    fairing::AdHoc,
    serde::json::Json,
    State,
};
use rocket_sync_db_pools::database;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
#[serde(crate = "rocket::serde")]
pub struct ApiError {
    pub details: String,
}

#[macro_use]
extern crate rocket;

#[macro_use]
extern crate diesel;

#[database("db1")]
pub struct Db1(diesel::PgConnection);

#[database("db2")]
pub struct Db2(diesel::PgConnection);

#[derive(Deserialize)]
struct Config {
    name: String,
    age: u8,
}

table! {
    blog_posts (id) {
        id -> Int4,
        title -> Varchar,
        body -> Text,
        published -> Bool,
    }
}

#[derive(Serialize, Deserialize, Queryable, AsChangeset, Debug, Insertable, Eq)]
#[table_name = "blog_posts"]
struct BlogPost {
    id: i32,
    title: String,
    body: String,
    published: bool,
}

impl PartialEq for BlogPost {
    fn eq(&self, other: &Self) -> bool {
        self.title == other.title
    }
}

impl Ord for BlogPost {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.title.cmp(&other.title)
    }
}

impl PartialOrd for BlogPost {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[get("/db1")]
async fn get_all_blog_posts1(connection: Db1) -> Json<Vec<BlogPost>> {
    connection
        .run(|c| blog_posts::table.load(c))
        .await
        .map(Json)
        .expect("Failed to fetch blog posts")
}

#[get("/db2")]
async fn get_all_blog_posts2(connection: Db2) -> Json<Vec<BlogPost>> {
    connection
        .run(|c| blog_posts::table.load(c))
        .await
        .map(Json)
        .expect("Failed to fetch blog posts")
}

#[get("/")]
async fn get_all_blog_posts(connection1: Db1, connection2: Db2) -> Json<Vec<BlogPost>> {
    let v1: Vec<BlogPost> = connection1
        .run(|c| blog_posts::table.load(c))
        .await
        .expect("abc");
    let v2: Vec<BlogPost> = connection2
        .run(|c| blog_posts::table.load(c))
        .await
        .expect("def");

    let mut data = Vec::new();
    data.extend(v1);
    for item2 in v2 {
        if !data.contains(&item2) {
            data.push(item2);
        }
    }
    data.sort();
    Json(data)
}

#[get("/random")]
fn get_random_blog_post() -> Json<BlogPost> {
    Json(BlogPost {
        id: 1,
        title: "My first post".to_string(),
        body: "This is my first post".to_string(),
        published: true,
    })
}

#[get("/<id>")]
fn get_blog_post(id: i32) -> Json<BlogPost> {
    Json(BlogPost {
        id,
        title: "Some title".to_string(),
        body: "Some body".to_string(),
        published: true,
    })
}

#[post("/db1", data = "<blog_post>")]
async fn create_blog_post1(connection: Db1, blog_post: Json<BlogPost>) -> Json<BlogPost> {
    connection
        .run(move |c| {
            diesel::insert_into(blog_posts::table)
                .values(&blog_post.into_inner())
                .get_result(c)
        })
        .await
        .map(Json)
        .expect("boo")
}

#[delete("/db1/delete/<id>")]
async fn destroy1(connection: Db1, id: i32) -> Result<NoContent, NotFound<Json<ApiError>>> {
    connection
        .run(move |c| {
            // let affected = diesel::delete(blog_posts::table.filter(blog_posts::title.eq(title)))
            let affected = diesel::delete(blog_posts::table.filter(blog_posts::id.eq(id)))
                .execute(c)
                .expect("Connection is broken");
            match affected {
                1 => Ok(()),
                0 => Err("NotFound"),
                _ => Err("???"),
            }
        })
        .await
        .map(|_| NoContent)
        .map_err(|e| {
            NotFound(Json(ApiError {
                details: e.to_string(),
            }))
        })
}

#[post("/db2", data = "<blog_post>")]
async fn create_blog_post2(connection: Db2, blog_post: Json<BlogPost>) -> Json<BlogPost> {
    connection
        .run(move |c| {
            diesel::insert_into(blog_posts::table)
                .values(&blog_post.into_inner())
                .get_result(c)
        })
        .await
        .map(Json)
        .expect("boo")
}

#[delete("/db2/delete/<id>")]
async fn destroy2(connection: Db2, id: i32) -> Result<NoContent, NotFound<Json<ApiError>>> {
    connection
        .run(move |c| {
            // let affected = diesel::delete(blog_posts::table.filter(blog_posts::title.eq(title)))
            let affected = diesel::delete(blog_posts::table.filter(blog_posts::id.eq(id)))
                .execute(c)
                .expect("Connection is broken");
            match affected {
                1 => Ok(()),
                0 => Err("NotFound"),
                _ => Err("???"),
            }
        })
        .await
        .map(|_| NoContent)
        .map_err(|e| {
            NotFound(Json(ApiError {
                details: e.to_string(),
            }))
        })
}

#[get("/config")]
fn get_config(config: &State<Config>) -> String {
    format!("Hello, {}! You are {} years old.", config.name, config.age)
}

#[launch]
fn rocket() -> _ {
    let rocket = rocket::build();
    rocket
        .attach(Db1::fairing())
        .attach(Db2::fairing())
        .attach(AdHoc::config::<Config>())
        .mount("/", routes![index, get_config])
        .mount(
            "/blog-posts",
            routes![
                get_random_blog_post,
                get_blog_post,
                get_all_blog_posts,
                get_all_blog_posts1,
                get_all_blog_posts2,
                create_blog_post1,
                create_blog_post2,
                destroy1,
                destroy2
            ],
        )
}
