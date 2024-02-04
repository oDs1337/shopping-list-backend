mod api;
mod models;
mod repository;

#[macro_use]
extern crate rocket;
use rocket::{get, http::Status, serde::json::Json};
use api::shopping_list_api::{add_item, get_item, get_all_items, delete_item};
use repository::mongodb_repo::MongoRepo;
use rocket_cors::{AllowedOrigins};

#[launch]
fn rocket() -> _ {
    let allowed_origins = AllowedOrigins::some_exact(&["http://localhost:4200"]);
    let cors = rocket_cors::CorsOptions {
        allowed_origins,
        ..Default::default()
    };

    let db = MongoRepo::init();
    rocket::build()
        .attach(cors.to_cors().unwrap())
        .manage(db)
        .mount("/", routes![add_item])
        .mount("/", routes![get_item])
        .mount("/", routes![get_all_items])
        .mount("/", routes![delete_item])
}