#![feature(plugin)]
#![feature(custom_derive)]
#![plugin(rocket_codegen)]

extern crate chrono;
extern crate serde;
extern crate rocket;
extern crate rocket_contrib;
extern crate rusqlite;

use std::collections::HashMap;
use chrono::Utc;
#[macro_use] extern crate serde_derive;
use rocket::response::Redirect;
use rocket::request::Form;
use rocket_contrib::Template;
use rusqlite::Connection;

#[derive(FromForm, Serialize)]
struct Post {
    reply_id: String,
    name: String,
    title: String,
    content: String,
}

#[derive(Serialize)]
struct IndexData{
    title: String,
    announcement: String,
    posts: Vec<Post>,
}

#[get("/")]
fn index() -> Template {
    let database_url = "db/guestbook.db";
    let conn = Connection::open(database_url).unwrap();
    let mut stmt = conn.prepare("SELECT name, title, content FROM post").unwrap();
    let post_iter = stmt.query_map(&[], |row| {
        Post {
               name: row.get(0),
              title: row.get(1),
            content: row.get(2),
        }
    }).unwrap();

    let context = IndexData {
        title: "Rust GuestBook".to_string(),
        announcement: "Welcome to my guestbook.".to_string(),
        posts: post_iter.map(|post| post.unwrap()).collect(),
    };

    Template::render("index", context)
}

#[get("/topic_form")]
fn topic_form() -> Template {
    let mut context = HashMap::new();
    context.insert("title", "新增留言");
    Template::render("post_form", context)
}

#[get("/reply_form/<reply_id>")]
fn topic_form() -> Template {
    let mut context = HashMap::new();
    context.insert("title", "回覆留言".to_string());
    context.insert("reply_id", reply_id.to_string());
    Template::render("post_form", context)
}


#[post("/post", data="<post>")]
fn create_topic(post: Form<Post>) -> Redirect {
    let database_url = "db/guestbook.db";
    let post_data = post.get();
    let conn = Connection::open(database_url).unwrap();
    conn.execute("INSERT INTO post (reply_id, name, title, content, created_time) VALUES (?1, ?2, ?3, ?4, ?5)",
                 &[&post_data.reply_id, &post_data.name, &post_data.title, &post_data.content, &Utc::now().naive_utc().to_string()]).unwrap();
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
