#![feature(plugin)]
#![plugin(rocket_codegen)]

extern crate rocket;

#[get("/")]
fn index() -> &'static str {
    "benita"
}

fn main() {
    rocket::ignite().mount("/", routes![index]).launch();
}
