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
use models::NewRustacean;
use models::Rustacean;
use rocket::response::status;
use rocket::serde::json::{json, Json, Value};

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
async fn view_rustacean(id: i32, _auth: BasicAuth, db: DbConn) -> Value {
    db.run(move |c| {
        let result = schema::rustaceans::table
            .find(id)
            .get_result::<Rustacean>(c)
            .expect("Error loading rustacean");
        json!(result)
    })
    .await
}

#[post("/rustaceans", format = "json", data = "<new_rustacean>")]
async fn create_rustacean(
    _auth: BasicAuth,
    db: DbConn,
    new_rustacean: Json<NewRustacean>,
) -> Value {
    db.run(|c| {
        let result = diesel::insert_into(schema::rustaceans::table)
            .values(new_rustacean.into_inner())
            .execute(c)
            .expect("Error saving new rustacean");
        json!(result)
    })
    .await
}

#[put("/rustaceans/<id>", format = "json", data = "<rustacean>")]
async fn update_rustacean(
    id: i32,
    _auth: BasicAuth,
    db: DbConn,
    rustacean: Json<Rustacean>,
) -> Value {
    db.run(move |c| {
        let result = diesel::update(schema::rustaceans::table.find(id))
            .set((
                schema::rustaceans::name.eq(rustacean.name.to_owned()),
                schema::rustaceans::email.eq(rustacean.email.to_owned()),
            ))
            .execute(c)
            .expect("Error updating rustacean");
        json!(result)
    })
    .await
}

#[delete("/rustaceans/<id>")]
async fn delete_rustacean(id: i32, _auth: BasicAuth, db: DbConn) -> status::NoContent {
    db.run(move |c| {
        diesel::delete(schema::rustaceans::table.find(id))
            .execute(c)
            .expect("Error deleting rustacean");
        status::NoContent
    })
    .await
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
