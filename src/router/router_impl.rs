use actix_web::{web, HttpResponse};
use crate::router::v1::init_test_route;

pub fn init_router(cfg: &mut web::ServiceConfig) {
  cfg
    .service(
      web::scope("/v1")
      .service(web::resource("/health").to(get_health_status))
      .configure(init_test_route)
    );
}

async fn get_health_status() -> HttpResponse {
  HttpResponse::Ok()
      .content_type("application/json")
      .body("Healthy!")
}