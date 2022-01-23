#[macro_use]
extern crate rocket;

use rocket::http::CookieJar;
use rocket::Build;
use rocket::Rocket;

#[get("/")]
fn index() -> &'static str {
    "hello world"
}

#[get("/cookie")]
fn cookie(cookies: &CookieJar<'_>) -> Option<String> {
    cookies
        .get("message")
        .map(|crumb| format!("Message: {}", crumb.value()))
}

#[launch]/
fn rocket() -> Rocket<Build> {
    rocket::build()
        .mount("/", routes![index])
        .mount("/", routes![cookie])
}
