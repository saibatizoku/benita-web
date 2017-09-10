#[get("/")]
fn ph_index() -> &'static str {
    "ph"
}

#[get("/info")]
fn get_ph_info() -> &'static str {
    "ph info"
}

#[get("/led")]
fn get_ph_led() -> &'static str {
    "ph led"
}

#[get("/reading")]
fn get_ph_reading() -> &'static str {
    "ph reading"
}
