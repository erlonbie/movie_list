use diesel::prelude::*;

use api::{
    models::{BlogPost, NewBlogPost},
    schema::blog_posts,
    ApiError, Config, Db1, Db2,
};
use rocket::{
    fairing::{AdHoc, Fairing, Info, Kind},
    http::Header,
    // fairing::AdHoc,
    response::status::{NoContent, NotFound},
    serde::json::Json,
    Request,
    Response,
    State,
};

pub struct CORS;
#[rocket::async_trait]
impl Fairing for CORS {
    fn info(&self) -> Info {
        Info {
            name: "Add CORS headers to responses",
            kind: Kind::Response,
        }
    }

    async fn on_response<'r>(&self, _req: &'r Request<'_>, response: &mut Response<'r>) {
        response.set_header(Header::new("Access-Control-Allow-Origin", "*"));
        response.set_header(Header::new(
            "Access-Control-Allow-Methods",
            "POST, GET, PATCH, OPTIONS, DELETE",
        ));
        response.set_header(Header::new("Access-Control-Allow-Headers", "*"));
        response.set_header(Header::new("Access-Control-Allow-Credentials", "true"));
    }
}

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
        } else {
            let mut movie = data.
                iter_mut().
                find(|p| { p.title == item2.title })
                .unwrap();
            movie.body = "both".to_string();
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
async fn create_blog_post1(connection: Db1, blog_post: Json<NewBlogPost>) -> Json<BlogPost> {
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

#[rocket::options("/db1/delete/<title>")]
async fn destroy1_option(connection: Db1, title: String) -> Result<NoContent, NotFound<Json<ApiError>>> {
    connection
        .run(move |c| {
            // let affected = diesel::delete(blog_posts::table.filter(blog_posts::title.eq(title)))
            let affected = diesel::delete(blog_posts::table.filter(blog_posts::title.eq(&title)))
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

#[rocket::delete("/db1/delete/<title>")]
async fn destroy1(connection: Db1, title: String) -> Result<NoContent, NotFound<Json<ApiError>>> {
    connection
        .run(move |c| {
            // let affected = diesel::delete(blog_posts::table.filter(blog_posts::title.eq(title)))
            let affected = diesel::delete(blog_posts::table.filter(blog_posts::title.eq(&title)))
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
async fn create_blog_post2(connection: Db2, blog_post: Json<NewBlogPost>) -> Json<BlogPost> {
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

#[rocket::options("/db2/delete/<title>")]
async fn destroy2_option(connection: Db2, title: String) -> Result<NoContent, NotFound<Json<ApiError>>> {
    connection
        .run(move |c| {
            // let affected = diesel::delete(blog_posts::table.filter(blog_posts::title.eq(title)))
            let affected = diesel::delete(blog_posts::table.filter(blog_posts::title.eq(&title)))
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

#[rocket::delete("/db2/delete/<title>")]
async fn destroy2(connection: Db2, title: String) -> Result<NoContent, NotFound<Json<ApiError>>> {
    connection
        .run(move |c| {
            // let affected = diesel::delete(blog_posts::table.filter(blog_posts::title.eq(title)))
            let affected = diesel::delete(blog_posts::table.filter(blog_posts::title.eq(&title)))
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
        .attach(CORS)
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
                destroy1_option,
                destroy2,
                destroy2_option
            ],
        )
}
