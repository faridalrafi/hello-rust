#![feature(plugin)] // use plugin
#![plugin(rocket_codegen)] //use plugin

extern crate rocket;
extern crate rocket_contrib;

#[cfg(test)] mod tests;

use rocket::Request;
use rocket_contrib::Template;

#[get("/")]
fn index() -> &'static str {
    "Hello world!"
}

#[get("/hello/<name>")]
fn hello(name: String) -> String {
    format!("Hey, {}", name)
}

#[catch(404)]
fn not_found(req: &Request) -> Template {
    let mut map = std::collections::HashMap::new();
    map.insert("path", req.uri().as_str());
    Template::render("error/404", &map)
}

fn rocket() -> rocket::Rocket {
    rocket::ignite()
        .mount("/", routes![index, hello])
        .attach(Template::fairing())
        .catch(catchers![not_found])
}

fn main() {
    rocket().launch();
}
