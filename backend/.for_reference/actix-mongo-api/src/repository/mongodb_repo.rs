use std::env;
extern crate dotenv;
use dotenv::dotenv;

use mongodb::{
    bson::{extjson::de::Error, oid::ObjectId, doc, self},
    results::{InsertOneResult},
    Client, Collection, options::FindOneOptions,
};
use crate::models::prompt_model::Prompt;

pub struct MongoRepo {
    collection: Collection<Prompt>,
}

impl MongoRepo {
    pub async fn init() -> Self {
        dotenv().ok();
        let mongo_url = match env::var("MONGO_URL") {
            Ok(val) => val,
            Err(_) => panic!("MONGO_URL must be set"),
        };
        let client = Client::with_uri_str(mongo_url.as_str()).await.unwrap();
        let db = client.database("generative_AI");
        let col: Collection<Prompt> = db.collection("prompthero");
        MongoRepo { collection: col }
    }

    pub async fn get_prompt(&self, id: String) -> Result<Prompt, Error> {
        let obj_id = ObjectId::parse_str(id).unwrap();

        let filter = doc! {"_id": obj_id};
        let projection: Option<FindOneOptions> = Some(FindOneOptions::builder().projection(doc! {"prompt_url": 1, "artist_name": 1, "model_used": 1}).build());

        let result = self.collection.find_one(filter, projection).await.ok().expect("Failed to get prompt");
        Ok(result.unwrap())
    }
}