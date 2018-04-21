#![feature(plugin)]
#![feature(custom_derive)]
#![plugin(rocket_codegen)]

extern crate chrono;
extern crate rocket;
extern crate rocket_contrib;
extern crate rusqlite;

use std::collections::HashMap;
use chrono::Utc;
use rocket::response::Redirect;
use rocket::request::Form;
use rocket_contrib::Template;
use rusqlite::Connection;

#[derive(FromForm)]
struct Post {
    name: String,
    title: String,
    content: String,
}

#[get("/")]
fn index() -> Template {
    let mut context = HashMap::new();
    context.insert("title", "Rust GuestBook");
    context.insert("index_content", "Welcome to my guestbook.");

    let database_url = "db/guestbook.db";
    let post_data = post.get();
    let conn = Connection::open(database_url).unwrap();
    let mut stmt = conn.prepare("SELECT name, title, content FROM post").unwrap();
    let post_iter = stmt.query_map(&[], |row| {
        Post {
               name: row.get(0),
              title: row.get(1),
            content: row.get(2),
        }
    }).unwrap();

    let mut post_content = String::new();
    for post in post_iter {
        let mut post_context = HashMap::new();
        post_context.insert("name", &post.name);
        post_context.insert("title", &post.title);
        post_context.insert("content", &post.content);
        post_content.push_str(Template::show("templates/", "post", post_context));
    }
    context.insert("posts", post_content);

    Template::render("index", context)
}

#[get("/topic_form")]
fn topic_form() -> Template {
    let mut context = HashMap::new();
    context.insert("title", "Rust GuestBook");
    Template::render("topic_form", context)
}

#[post("/topic", data="<post>")]
fn create_topic(post: Form<Post>) -> Redirect {
    let database_url = "db/guestbook.db";
    let post_data = post.get();
    let conn = Connection::open(database_url).unwrap();
    conn.execute("INSERT INTO post (name, title, content, created_time) VALUES (?1, ?2, ?3, ?4)",
                 &[&post_data.name, &post_data.title, &post_data.content, &Utc::now().naive_utc().to_string()]).unwrap();
    Redirect::to("/")
}

fn main() {
    rocket::ignite()
      .mount("/", routes![index])
      .mount("/", routes![topic_form])
      .mount("/", routes![create_topic])
      .attach(Template::fairing())
      .launch();
}
