use super::rocket;
use rocket::local::Client;
use rocket::http::Status;

fn client() -> Client {
    Client::new(rocket::ignite().mount("/", routes![super::index, super::hello])).unwrap()
}

fn test(uri: &str, expected: String) {
    let client = client();
    assert_eq!(client.get(uri).dispatch().body_string(), Some(expected));
}

// unit testing for index
#[test]
fn test_index() {
    test("/", "Hello world!".to_string())
}

// unit testing for not found page
#[test]
fn test_not_found() {
    let client = client();
    let response = client.get("/wkwkwk").dispatch().status();
    assert_eq!(response, Status::NotFound);
}

// unit testing got hello page
#[test]
fn test_hello() {
    for name in &["ngerpin", "ngurpun"] {
        test(&format!("/hello/{}", name), format!("Hey, {}", name));    
    }
}