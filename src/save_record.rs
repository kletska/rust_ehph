use std::sync::Arc;

use actix_web::{HttpRequest, HttpResponse, Responder, web};
use serde::Deserialize;

use crate::repo::{Record, MongoRepo};

pub async fn save_record(req: web::Json<Record>) -> impl Responder {

    let db =  MongoRepo::new().await.unwrap();
    db.save_record(req.0).await;
    HttpResponse::Ok()
}