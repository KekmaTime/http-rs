#[macro_use]
extern crate rocket;

use rocket::response::status;
use rocket::serde::json::{json, Value};

pub struct BasicAuth {
    pub username: String,
    pub password: String,
}

impl BasicAuth {
    fn from_authorization_header(header: &str) -> Option<BasicAuth> {
        let split = header.split_whitespace().collect::<Vec<&str>>();
        if split.len() != 2 || split[0] != "Basic" {
            return None;
        }
        Self::from_base64_encoded(split[1])
    }

    fn from_base64_encoded(base64_string: &str) -> Option<BasicAuth> {
        let decoded = base64::decode(base64_string).ok()?;
        let decoded = String::from_utf8(decoded).ok()?;
        let split = decoded.split(":").collect::<Vec<&str>>();
        if split.len() != 2 {
            return None;
        }

        let (username, password) = (split[0].to_string(), split[1].to_string());

        Some(BasicAuth { username, password })
    }
}

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
