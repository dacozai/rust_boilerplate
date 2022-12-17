use actix_cors::Cors;
use actix_web::{http::header};

// reference -- https://stackoverflow.com/questions/73098788/access-control-allow-origin-missing-using-actix-web
pub fn cors() -> Cors {
  Cors::default()
    .allowed_origin("http://localhost:377")
    .allowed_origin("http://127.0.0.1:377")
    .allowed_methods(vec!["GET", "POST"])
    .allowed_headers(vec![header::AUTHORIZATION, header::ACCEPT])
    .allowed_header(header::CONTENT_TYPE)
    .max_age(3600)
}