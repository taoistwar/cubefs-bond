mod controller;

#[macro_use]
extern crate rocket;

#[get("/")]
fn index() -> &'static str {
    r#"
        API  URI: /mount
             BODY: {}

    body is json, and for cubefs client config.json content
"#
}
#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index, controller::mount, controller::umount])
}
