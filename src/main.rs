#![feature(plugin)] // use plugin
#![plugin(rocket_codegen)] //use plugin

extern crate rocket;

#[cfg(test)] mod tests;

#[get("/")]
fn index() -> &'static str {
    "Hello world!"
}

#[get("/hello/<name>")]
fn hello(name: String) -> String {
    format!("Hey, {}", name)
}

#[catch(404)]
fn not_found(req: &rocket::Request) -> String {
    format!("Sorry, but '{}' is not a valid path!", req.uri())
}

fn rocket() -> rocket::Rocket {
    rocket::ignite()
        .mount("/", routes![index, hello])
        .catch(catchers![not_found])
}

fn main() {
    rocket().launch();
}
