use actix_web::{HttpRequest, HttpResponse, Responder};

pub async fn get_rating(req: HttpRequest) -> impl Responder {
    HttpResponse::Ok().body("data")
}