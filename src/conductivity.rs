#[get("/")]
fn conductivity_index() -> &'static str {
    "conductivity"
}

#[get("/info")]
fn get_conductivity_info() -> &'static str {
    "conductivity info"
}

#[get("/led")]
fn get_conductivity_led() -> &'static str {
    "conductivity led"
}

#[get("/reading")]
fn get_conductivity_reading() -> &'static str {
    "conductivity reading"
}
