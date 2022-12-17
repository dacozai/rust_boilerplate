use actix_web::{
  dev::{forward_ready, Service, ServiceRequest, ServiceResponse, Transform},
  Error,
};
use std::future::{ready, Ready};
use futures_util::future::LocalBoxFuture;

pub struct Greet;

impl<S, B> Transform<S, ServiceRequest> for Greet
where
  S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
  S::Future: 'static,
{
  type Response = ServiceResponse<B>;
  type Error = Error;
  type InitError = ();
  type Transform = SayHiMiddleware<S>;
  type Future = Ready<Result<Self::Transform, Self::InitError>>;

  fn new_transform(&self, service: S) -> Self::Future {
    ready(Ok(SayHiMiddleware { service }))
  }
}
pub struct SayHiMiddleware<S> {
  service: S,
}

impl<S, B> Service<ServiceRequest> for SayHiMiddleware<S>
where
  S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
  S::Future: 'static,
{
  type Response = ServiceResponse<B>;
  type Error = Error;
  type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

  forward_ready!(service);

  fn call(&self, req: ServiceRequest) -> Self::Future {
    println!("Hi from start. You requested: {}", req.path()); 

    let fut = self.service.call(req);

    Box::pin(async move {
        let res = fut.await?;
        Ok(res)
    })
  }
}