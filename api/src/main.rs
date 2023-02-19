use diesel::prelude::*;

use rocket::{
    fairing::AdHoc,
    response::status::{NoContent, NotFound},
    serde::json::Json,
    State,
};
use api::{models::BlogPost, schema::blog_posts, ApiError, Config, Db1, Db2};

#[rocket::get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[rocket::get("/db1")]
async fn get_all_blog_posts1(connection: Db1) -> Json<Vec<BlogPost>> {
    connection
        .run(|c| blog_posts::table.load(c))
        .await
        .map(Json)
        .expect("Failed to fetch blog posts")
}

#[rocket::get("/db2")]
async fn get_all_blog_posts2(connection: Db2) -> Json<Vec<BlogPost>> {
    connection
        .run(|c| blog_posts::table.load(c))
        .await
        .map(Json)
        .expect("Failed to fetch blog posts")
}

#[rocket::get("/")]
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

#[rocket::get("/random")]
fn get_random_blog_post() -> Json<BlogPost> {
    Json(BlogPost {
        id: 1,
        title: "My first post".to_string(),
        body: "This is my first post".to_string(),
        published: true,
    })
}

#[rocket::get("/<id>")]
fn get_blog_post(id: i32) -> Json<BlogPost> {
    Json(BlogPost {
        id,
        title: "Some title".to_string(),
        body: "Some body".to_string(),
        published: true,
    })
}

#[rocket::post("/db1", data = "<blog_post>")]
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

#[rocket::delete("/db1/delete/<id>")]
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

#[rocket::post("/db2", data = "<blog_post>")]
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

#[rocket::delete("/db2/delete/<id>")]
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

#[rocket::get("/config")]
fn get_config(config: &State<Config>) -> String {
    format!("Hello, {}! You are {} years old.", config.name, config.age)
}

#[rocket::launch]
fn rocket() -> _ {
    let rocket = rocket::build();
    rocket
        .attach(Db1::fairing())
        .attach(Db2::fairing())
        .attach(AdHoc::config::<Config>())
        .mount("/", rocket::routes![index, get_config])
        .mount(
            "/blog-posts",
            rocket::routes![
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
