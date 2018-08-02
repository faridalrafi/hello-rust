#![feature(plugin)] // use plugin
#![plugin(rocket_codegen)] //use plugin

extern crate rocket;

#[get("/")]
fn index() -> &'static str {
    "Hello world!"
}

#[catch(404)]
fn not_found(req: &rocket::Request) -> String {
    format!("Sorry, but '{}' is not a valid path!", req.uri())
}

fn rocket() -> rocket::Rocket {
    rocket::ignite()
        .mount("/", routes![index])
        .catch(catchers![not_found])
}

fn main() {
    rocket().launch();
}
