use std::fmt;

use rocket::{
    data::{self, FromData},
    response::stream::TextStream,
    serde, Data, Request,
};
use serde_json;

#[macro_use]
extern crate rocket;

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}
use serde_json::Value;
#[post("/bond/<volume>", data = "<input>")] // <- route attribute
fn bond(volume: &str, input: &str) -> String {
    println!("{}", input);
    let tmp: Value = match serde_json::from_str(input) {
        Ok(text) => text,
        Err(msg) => return String::from("erro"),
    };
    let json = match serde_json::to_string_pretty(tmp) {
        Ok(text) => text,
        Err(msg) => return msg.to_string(),
    };
    format!("volume:{}", volume)
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index, bond])
}
