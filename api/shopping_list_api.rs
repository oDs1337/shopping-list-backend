use crate::{models::item_model::Item, repository::mongodb_repo::MongoRepo};
use mongodb::results::InsertOneResult;
use rocket::{http::Status, serde::json::Json, State};

#[post("/add_item", data = "<new_item>")]
pub fn add_item(
    db: &State<MongoRepo>,
    new_item: Json<Item>,
) -> Result<Json<InsertOneResult>, Status> {
    let data = Item {
        id: None,
        item_name: new_item.item_name.to_owned(),
        item_price: new_item.item_price.to_owned(),
    };
    let item_detail = db.add_item(data);
    match item_detail {
        Ok(item) => Ok(Json(item)),
        Err(_) => Err(Status::InternalServerError),
    }
}

#[get("/get_item/<path>")]
pub fn get_item(db: &State<MongoRepo>, path: String) -> Result<Json<Item>, Status>{
    let id = path;
    if id.is_empty(){
        return Err(Status::BadRequest);
    };
    let item_detail = db.get_item(&id);
    match item_detail {
        Ok(item) => Ok(Json(item)),
        Err(_) => Err(Status::InternalServerError),
    }
}

#[get("/get_all_items")]
pub fn get_all_items(db: &State<MongoRepo>) -> Result<Json<Vec<Item>>, Status>{
    let item_detail = db.get_all_items();
    match item_detail {
        Ok(item) => Ok(Json(item)),
        Err(_) => Err(Status::InternalServerError),
    }
}
