#[macro_use]
extern crate rocket;

#[macro_use]
extern crate diesel;

#[macro_use]
extern crate rocket_sync_db_pools;
mod auth;
mod crud;
mod models;
mod schema;
use crud::*;

#[database("sqlite")]
struct DbConn(diesel::SqliteConnection);

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
