use askama::Template;
use serde::{Deserialize, Serialize};
use rusqlite::{Connection, Result as SqliteResult};
use actix_web::{web, App, HttpServer, HttpResponse, Responder, HttpRequest, Error};
use actix_web::error::{ErrorInternalServerError};

use std::fmt;
impl fmt::Display for Post {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "@{} {}", self.handle.as_deref().unwrap_or("anonymous"), self.content)
    }
}

#[derive(Deserialize, Default)]
struct QueryParams {
    handle: Option<String>,
}
#[derive(Serialize, Deserialize)]
struct Post {
    id: i32,
    handle: Option<String>,
    content: String,
}

#[derive(Template)]
#[template(path = "index.html")]
struct IndexTemplate {
    posts: Vec<Post>,
}

#[derive(Template)]
#[template(path = "post.html")]
struct PostTemplate {}

fn init_db() -> SqliteResult<()> {
    let conn = Connection::open("sqlite.db")?;
    conn.execute(
        "CREATE TABLE IF NOT EXISTS posts (
            id INTEGER PRIMARY KEY,
            handle TEXT NOT NULL,
            content TEXT NOT NULL
        )",
        [],
    )?;
    Ok(())
}

async fn index() -> impl Responder {
    let conn = Connection::open("sqlite.db").unwrap();
    let mut stmt = conn.prepare("SELECT id, handle, content FROM posts ORDER BY id DESC").unwrap();
    let posts = stmt.query_map([], |row| {
        Ok(Post {
            id: row.get(0)?,
            handle: row.get(1)?,
            content: row.get(2)?,
        })
    }).unwrap().filter_map(Result::ok).collect::<Vec<Post>>();

    let template = IndexTemplate { posts };
    HttpResponse::Ok().body(template.render().unwrap())
}

async fn post_form() -> impl Responder {
    let template = PostTemplate {};
    HttpResponse::Ok().body(template.render().unwrap())
}

#[derive(Deserialize)]
struct FormData {
    handle: String,
    content: String,
}

async fn submit_post(
    req: HttpRequest,
    form: web::Form<FormData>
) -> Result<impl Responder, Error> {
    let query = web::Query::<QueryParams>::from_query(req.query_string())
        .unwrap_or_else(|_| web::Query(QueryParams { handle: None }));
    //let handle = query.handle.clone().filter(|h| !h.is_empty());
    //println!("Debug: Extracted handle: {:?}", handle);
let handle = if !form.handle.is_empty() {
    form.handle.clone()
} else {
    query.handle.clone().unwrap_or_default()
};

println!("Debug: Using handle: {:?}", handle);

    let conn = Connection::open("sqlite.db")
        .map_err(|_| ErrorInternalServerError("Database connection failed"))?;
    let result = conn.execute(
        "INSERT INTO posts (handle, content) VALUES (?1, ?2)",
        &[&form.handle, &form.content],
    );
    match result {
 Ok(_) => {
     let redirect_url = if !handle.is_empty() {
         format!("/?handle={}", handle)
     } else {
         "/".to_string()
     };
     Ok(HttpResponse::SeeOther()
        .append_header(("Location",
                        redirect_url))
        .finish())
 },

        //Ok(_) => Ok(web::Redirect::to("/" + "?handle=" + handle).see_other()),
        Err(_) => Err(ErrorInternalServerError("Failed to insert post")),
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    init_db().unwrap();

    HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(index))
            .route("/post", web::get().to(post_form))
            .route("/submit", web::post().to(submit_post))
    })
    .bind("0.0.0.0:8080")?
    .run()
    .await
}
