#[macro_use] extern crate rocket;


use rocket::{get, http::Status, serde::json::Json};

#[get("/")]
fn index() -> Result<Json<String>, Status> {
    Ok(Json(String::from("Hello")))
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index])
}
