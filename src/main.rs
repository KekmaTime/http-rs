#[macro_use]
extern crate rocket;

#[macro_use]
extern crate diesel;

#[macro_use]
extern crate rocket_sync_db_pools;
mod auth;
mod models;
mod schema;
use auth::BasicAuth;
use diesel::prelude::*;
use models::Rustacean;
use rocket::response::status;
use rocket::serde::json::{json, Value};

#[database("sqlite")]

struct DbConn(diesel::SqliteConnection);

#[get("/rustaceans")]
async fn get_rustaceans(_auth: BasicAuth, db: DbConn) -> Value {
    db.run(|c| {
        let result = schema::rustaceans::table
            .limit(100)
            .load::<Rustacean>(c)
            .expect("Error loading rustaceans");
        json!(result)
    })
    .await
}

#[get("/rustaceans/<id>")]
fn view_rustacean(id: usize, _auth: BasicAuth) -> Value {
    json!([
        {
            "name": "John Doe",
            "age": 30,
            "address": {
                "street": "10 Downing Street",
                "city": "London"
            }
        },
        {
            "name": "Jane Doe",
            "age": 25,
            "address": {
                "street": "10 Downing Street",
                "city": "London"
            }
        }
    ])
}

#[post("/rustaceans", format = "json")]
fn create_rustacean(_auth: BasicAuth) -> Value {
    json!({
    "id": 3,
    "name": "John Doe",
    "age": 30,
    "address": {
        "street": "10 Downing Street",
        "city": "London"
        }
    })
}

#[put("/rustaceans/<id>", format = "json")]
fn update_rustacean(id: usize, _auth: BasicAuth) -> Value {
    json!({
    "id": id,
    "name": "John Doe",
    "age": 30,
    "address": {
        "street": "10 Downing Street",
        "city": "London"
        }
    })
}

#[delete("/rustaceans/<id>")]
fn delete_rustacean(id: usize, _auth: BasicAuth) -> status::NoContent {
    status::NoContent
}

#[catch(404)]
fn not_found() -> Value {
    json!({
        "status": "error",
        "reason": "Resource was not found."
    })
}

#[rocket::main]
async fn main() {
    let _ = rocket::build()
        .mount(
            "/",
            routes![
                get_rustaceans,
                view_rustacean,
                create_rustacean,
                update_rustacean,
                delete_rustacean
            ],
        )
        .attach(DbConn::fairing())
        .register("/", catchers![not_found])
        .launch()
        .await;
}
