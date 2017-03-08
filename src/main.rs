

#![feature(plugin)]
#![plugin(rocket_codegen)]

extern crate rocket;

#[get("/")]
fn index() -> &'static str {
    "Hello, This is from Geetika and avni!"
}

fn main() {
    rocket::ignite().mount("/", routes![index]).launch();
}
