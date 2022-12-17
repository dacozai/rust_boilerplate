use crate::err::CustomError;
use actix_web::{get, web, HttpResponse};
use lazy_static::lazy_static;
use crate::service::test_svc::TestSvc;
use crate::middleware::Greet as middlewareGreet;

lazy_static! {
  static ref TEST_SVC: TestSvc = TestSvc::new("world".to_string()); 
}

pub fn init_test_route(cfg: &mut web::ServiceConfig) {
  cfg
    .service(
      web::scope("/test")
        .service(test_func)
        .service(test_svc)
        .service(read_db)
    )
    .service(
      web::scope("/test_again")
      .wrap(middlewareGreet) 
      .service(greet_sumup)
    );
}

#[get("")]
async fn test_func() -> Result<HttpResponse, CustomError> {
  Ok(HttpResponse::Ok().json("this is for testing"))
}

#[get("/hello")]
async fn test_svc() -> Result<HttpResponse, CustomError> {
  let greeting = format!("TEST_SVC: {}", TEST_SVC.hello());
  Ok(HttpResponse::Ok().json(greeting))
}

#[get("")]
async fn greet_sumup() -> Result<HttpResponse, CustomError> {
  Ok(HttpResponse::Ok().json(TEST_SVC.sum_up(3, 7)))
}

#[get("/read_db")]
async fn read_db() -> Result<HttpResponse, CustomError> {
  Ok(HttpResponse::Ok().json(TEST_SVC.read_db()))
}
