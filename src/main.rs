#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;
#[macro_use] extern crate rocket_contrib;
#[macro_use] extern crate diesel;
#[macro_use] extern crate serde;

pub mod models;
pub mod schema;

use diesel::prelude::*;
use models::{User, NewUser};
use schema::users::dsl::*;
use rocket_contrib::json::Json;

#[database("power_gram")]
pub struct DbConn(diesel::PgConnection);

#[get("/")]
fn index(conn: DbConn) -> Json<Vec<User>> {
    let results = users.limit(1).load::<User>(&*conn).expect("Error loading users");
    return Json(results);
}

#[post("/users", format = "json", data = "<user_params>")]
fn create_user(conn: DbConn, user_params: Json<NewUser>) -> Json<User> {
    let user = diesel::insert_into(users)
        .values(user_params.into_inner()) // TODO: Hash passwords
        .get_result(&*conn)
        .expect("Error saving new post");

    return Json(user);
}

fn main() {
    rocket::ignite()
        .attach(DbConn::fairing())
        .mount("/", routes![index, create_user]).launch();
}