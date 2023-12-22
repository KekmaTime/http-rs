#[macro_use]
extern crate rocket;

use rocket::response::status;
use rocket::serde::json::{json, Value};

#[get("/rustaceans")]
fn get_rustaceans() -> Value {
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

#[get("/rustaceans/<id>")]
fn view_rustacean(id: usize) -> Value {
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
fn create_rustacean() -> Value {
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
fn update_rustacean(id: usize) -> Value {
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
fn delete_rustacean(id: usize) -> status::NoContent {
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
        .register("/", catchers![not_found])
        .launch()
        .await;
}
