use std::{env, error::Error};

use mongodb::{Database, bson::{self, Bson, Document, doc}, options::{ClientOptions, FindOptions, ResolverConfig}};
use futures::stream::StreamExt;
use serde::{Deserialize, Serialize};
pub struct MongoRepo {
    database: Database,    
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Record {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<bson::oid::ObjectId>, 
    pub email: String,
    pub name: String,
    pub score: i32,
}

#[derive(Serialize, Deserialize)]
pub struct NamedRating {
    pub position: i32,
    pub records: Vec<Record>,
}

const default_database_name: &str = "race";
const default_collection_name: &str = "players";

impl MongoRepo {
    pub async fn new() -> Result<MongoRepo, Box<dyn Error>> {
        let client_uri = env::var("MONGODB_URI")
            .expect("Please set the MONGODB_URI environment var");

        let options =
            ClientOptions::parse_with_resolver_config(&client_uri, ResolverConfig::cloudflare())
            .await?;

        let client = mongodb::Client::with_options(options)?;

        Ok(MongoRepo {
            database: client.database(default_database_name),
        })
    }

    pub async fn save_record(&self, record: Record) {
        let collection = self.database.collection(default_collection_name);

        let update_result = collection.update_one(
            doc! {
                "email": record.email.clone(),
                "name": record.name.clone(),
                "score": doc! {
                    "$lt": record.score
                }
            },
            doc! {
                "$set": { "score": record.score }
            },
            None,
        ).await.unwrap();

        if let None = update_result.upserted_id {
            let _ = collection.insert_one(
                doc!{
                    "email": record.email,
                    "name": record.name,
                    "score": record.score,
                }, 
                None
            ).await.unwrap();
        }
    }

    pub async fn get_rating(&self) -> Vec<Record> {

        let collection = self.database.collection(default_collection_name);

        let mut cursor = collection.find(
            doc!{},
            FindOptions::builder().limit(15).sort(doc!{"score": -1}).build()
        ).await.unwrap();

        let mut res = Vec::new(); //cursor.collect().await;

        while let Some(doc) = cursor.next().await {
            res.push(bson::from_bson(Bson::Document(doc.unwrap())).unwrap());
        }

        res
    } 

    pub async fn get_named_raiting(&self, record: Record) -> NamedRating {

        let mut top = self.get_rating().await;

        let mut index = -1;

        for (i, rec) in top.iter().enumerate() {
            if rec.email == record.email && rec.name == record.name {
                index = i as i32;
                break;
            }
        }

        if index == -1 {
            let collection = self.database.collection(default_collection_name);

            let rec: Record = bson::from_bson(Bson::Document(collection.find_one(doc!{"email": record.email}, None).await.unwrap().unwrap())).unwrap();

            index = collection.count_documents(doc!{"score": doc!{"$gte": rec.score}}, None).await.unwrap() as i32;
        } else {
            index += 1;
        }
        
        NamedRating {
            records: top, 
            position: index,
        }
    }
}