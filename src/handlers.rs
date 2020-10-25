use crate::models::Status;
use actix_web::{web, Responder};

pub async fn status() -> impl Responder {
    web::HttpResponse::Ok().json(Status {
        status: "UP".to_string(),
    })
}
