use std::env;
extern crate dotenv;
use dotenv::dotenv;

use mongodb::{
    bson::{extjson::de::Error, oid::ObjectId, doc},
    results::{InsertOneResult, DeleteResult, UpdateResult},
    sync::{Client, Collection},
};

use crate::models::item_model::Item;

pub struct MongoRepo{
    col: Collection<Item>,
}

impl MongoRepo {
    pub fn init() -> Self{
        dotenv().ok();
        let uri = match env::var("MONGOURI") {
            Ok(v) => v.to_string(),
            Err(_) => format!("Error loading env variable"),
        };
        let client = Client::with_uri_str(uri).unwrap();
        let db = client.database("shopping_list_db");
        let col: Collection<Item> = db.collection("shopping_list_items");
        MongoRepo{col}
    }

    pub fn add_item(&self, add_item: Item) -> Result<InsertOneResult, Error> {
        let new_doc = Item {
            id: None,
            item_name: add_item.item_name,
            item_price: add_item.item_price,
        };
        let item = self
            .col
            .insert_one(new_doc, None)
            .ok()
            .expect("Error adding item");
        Ok(item)
    }

    pub fn get_item(&self, id: &String) -> Result<Item, Error>{
        let obj_id = ObjectId::parse_str(id).unwrap();
        let filter = doc! {"_id": obj_id};
        let item_detail = self
            .col
            .find_one(filter, None)
            .ok()
            .expect("Error getting item");
        Ok(item_detail.unwrap())
    }

    pub fn get_all_items(&self) -> Result<Vec<Item>, Error>{
        let cursors = self
            .col
            .find(None, None)
            .ok()
            .expect("Error getting list of users");
        let items = cursors.map(|doc| doc.unwrap()).collect();
        Ok(items)
    }

    pub fn delete_item(&self, id: &String) -> Result<DeleteResult, Error> {
        let obj_id = ObjectId::parse_str(id).unwrap();
        let filter = doc! {"_id": obj_id};
        let item_detail = self
            .col
            .delete_one(filter, None)
            .ok()
            .expect("Error deleting item");
        Ok(item_detail)
    }

    pub fn update_item(&self, id: &String, new_item: Item) -> Result<UpdateResult, Error> {
        let obj_id = ObjectId::parse_str(id).unwrap();
        let filter = doc! {"_id": obj_id};
        let new_doc = doc! {
                "$set":
                    {
                        "id": new_item.id,
                        "item_name": new_item.item_name,
                        "item_price": new_item.item_price
                    },
            };
        let updated_doc = self
            .col
            .update_one(filter, new_doc, None)
            .ok()
            .expect("Error updating item");
        Ok(updated_doc)
    }
}