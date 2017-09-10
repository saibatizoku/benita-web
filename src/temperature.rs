#[get("/")]
fn temperature_index() -> &'static str {
    "temperature"
}

#[get("/info")]
fn get_temperature_info() -> &'static str {
    "temperature info"
}

#[get("/led")]
fn get_temperature_led() -> &'static str {
    "temperature led"
}

#[get("/reading")]
fn get_temperature_reading() -> &'static str {
    "temperature reading"
}
