use std::sync::Arc;

use actix_web::{web, App, HttpRequest, HttpServer, Responder};

mod repo;
mod get_named_rating;
mod get_rating;
mod save_record;

use repo::Record;
use save_record::save_record;
use get_rating::get_rating;
use get_named_rating::get_named_rating;

use crate::repo::MongoRepo;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .route("/methods/saveRecord", web::post().to(save_record))
            .route("/methods/getRating", web::get().to(get_rating) )
            .route("/methods/getNamedRating", web::get().to(get_named_rating))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}