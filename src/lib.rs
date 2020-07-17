#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;
#[macro_use]
extern crate rocket_contrib;

use rocket_contrib::json::JsonValue;

use dotenv::dotenv;

mod config;

#[catch(404)]
fn not_found() -> JsonValue {
    json!({
        "status": "error",
        "reason": "Resource was not found."
    })
}

pub fn rocket() -> rocket::Rocket {
    dotenv().ok();
    rocket::custom(config::from_env())
        .mount(
            "/api",
            routes![],
        )
        .register(catchers![not_found])
}