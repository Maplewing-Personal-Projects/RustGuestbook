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
    id: Option<i32>,
    reply_id: Option<i32>,
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
    let mut stmt = conn.prepare("SELECT id, reply_id, name, title, content FROM post").unwrap();
    let post_iter = stmt.query_map(&[], |row| {
        Post {
                 id: row.get(0),
           reply_id: row.get(1),
               name: row.get(2),
              title: row.get(3),
            content: row.get(4),
        }
    }).unwrap();

    let context = IndexData {
        title: "Rust GuestBook".to_string(),
        announcement: "Welcome to my guestbook.".to_string(),
        posts: post_iter.map(|post| post.unwrap()).filter(|post| post.reply_id == None).collect(),
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
fn reply_form(reply_id: String) -> Template {
    let mut context = HashMap::new();
    context.insert("title", "回覆留言".to_string());
    context.insert("reply_id", reply_id);
    Template::render("post_form", context)
}


#[post("/post", data="<post>")]
fn create_post(post: Form<Post>) -> Redirect {
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
      .mount("/", routes![reply_form])
      .mount("/", routes![create_post])
      .attach(Template::fairing())
      .launch();
}
