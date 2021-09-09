#[macro_use]
extern crate rocket;

#[get("/")]
fn index() -> &'static str {
    "Hello  World"
}

#[get("/<name>")]
fn hello(name: String) -> String {
    format!("Hello {}", name)
}

#[launch]
fn launch() -> _ {
    rocket::build().mount("/", routes![index, hello])
}
