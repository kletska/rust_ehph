use actix_web::{HttpRequest, HttpResponse, Responder};
use qstring::QString;

use crate::repo::{MongoRepo, Record};

pub async fn get_named_rating(req: HttpRequest) -> impl Responder {

    let query_str = req.query_string();
    let qs = QString::from(query_str);

    let db = MongoRepo::new().await.unwrap();

    let record = Record {
        email: String::from(qs.get("email").unwrap()),
        name: String::from(qs.get("nickname").unwrap()),
        score: 0,
        id: None,
    };
    
    let res = db.get_named_raiting(record).await;

    HttpResponse::Ok()
        .json(res)
}