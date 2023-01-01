use rocket::{build, get, launch, routes};

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[launch]
fn rocket() -> _ {
    build().mount("/", routes![index])
}
