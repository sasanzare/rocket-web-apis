#[macro_use] extern crate rocket;
#[macro_use] extern crate rocket_sync_db_pools;
#[macro_use] extern crate diesel;




//add the modules
mod api; 
mod models;
mod repository;
mod schema;


use rocket::{get, http::Status, serde::json::Json};

#[get("/")]
fn index() -> Result<Json<String>, Status> {
    Ok(Json(String::from("Hello")))
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index])
}


