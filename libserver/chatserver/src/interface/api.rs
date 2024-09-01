use actix_web::{get, HttpResponse};
use actix_web::web::ServiceConfig;

pub fn config(cfg: &mut ServiceConfig) {
    cfg.service(health);
}

#[get("health")]
async fn health() -> HttpResponse {
    HttpResponse::Ok().json("hahha")
}


