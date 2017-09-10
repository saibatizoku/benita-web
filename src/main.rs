#![feature(plugin)]
#![plugin(rocket_codegen)]

extern crate rocket;

mod conductivity;
mod temperature;
mod ph;

#[get("/")]
fn index() -> &'static str {
    "benita v0.1.0"
}

fn main() {
    rocket::ignite()
        .mount("/", routes![index])
        .mount("/conductivity",
               routes![conductivity::conductivity_index,
                       conductivity::get_conductivity_info,
                       conductivity::get_conductivity_led,
                       conductivity::get_conductivity_reading,
                       ])
        .mount("/temperature",
               routes![temperature::temperature_index,
                       temperature::get_temperature_info,
                       temperature::get_temperature_led,
                       temperature::get_temperature_reading,
                       ])
        .mount("/ph",
               routes![ph::ph_index,
                       ph::get_ph_info,
                       ph::get_ph_led,
                       ph::get_ph_reading,
                       ])
        .launch();
}
