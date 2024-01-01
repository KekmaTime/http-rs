use super::auth::BasicAuth;
use super::models::{NewRustacean, Rustacean};
use super::schema::rustaceans;
use super::DbConn;
use diesel::prelude::*;
use rocket::response::status;
use rocket::serde::json::{json, Json, Value};

#[get("/rustaceans")]
pub async fn get_rustaceans(_auth: BasicAuth, db: DbConn) -> Value {
    db.run(|c| {
        let result = rustaceans::table
            .limit(20)
            .load::<Rustacean>(c)
            .expect("Error loading rustaceans");
        json!(result)
    })
    .await
}

#[get("/rustaceans/<id>")]
pub async fn view_rustacean(_auth: BasicAuth, id: i32, db: DbConn) -> Value {
    db.run(move |c| {
        let result = rustaceans::table
            .find(id)
            .get_result::<Rustacean>(c)
            .expect("Error loading rustacean");
        json!(result)
    })
    .await
}

#[post("/rustaceans", format = "json", data = "<new_rustacean>")]
pub async fn create_rustacean(
    _auth: BasicAuth,
    new_rustacean: Json<NewRustacean>,
    db: DbConn,
) -> Value {
    db.run(move |c| {
        let result = diesel::insert_into(rustaceans::table)
            .values(new_rustacean.into_inner())
            .execute(c)
            .expect("Error saving new rustacean");
        json!(result)
    })
    .await
}

#[put("/rustaceans/<id>", format = "json", data = "<rustacean>")]
pub async fn update_rustacean(
    _auth: BasicAuth,
    id: i32,
    rustacean: Json<Rustacean>,
    db: DbConn,
) -> Value {
    db.run(move |c| {
        let result = diesel::update(rustaceans::table.find(id))
            .set((
                rustaceans::name.eq(rustacean.name.to_owned()),
                rustaceans::email.eq(rustacean.email.to_owned()),
            ))
            .execute(c)
            .expect("Error updating rustacean");
        json!(result)
    })
    .await
}

#[delete("/rustaceans/<id>")]
pub async fn delete_rustacean(_auth: BasicAuth, id: i32, db: DbConn) -> status::NoContent {
    db.run(move |c| {
        diesel::delete(rustaceans::table.find(id))
            .execute(c)
            .expect("Error deleting rustacean");
        status::NoContent
    })
    .await
}

#[catch(404)]
pub fn not_found() -> Value {
    json!({
        "status": "error",
        "reason": "Resource was not found."
    })
}
