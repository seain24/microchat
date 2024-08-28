use actix_web::{get, HttpResponse};
use actix_web::web::ServiceConfig;

pub fn register(cfg: &mut ServiceConfig) {
    cfg.service(health);
}

#[get("health")]
async fn health() -> HttpResponse {
    HttpResponse::Ok().json("hahha")
}
