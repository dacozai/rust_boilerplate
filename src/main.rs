extern crate env_logger;
use actix_web::{App, middleware::Logger, HttpServer};
use rust_boilerplate::*;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    cfg::init_config().await;
    db::sql::mysql::init();

    env_logger::init_from_env(env_logger::Env::new()
        .default_filter_or(cfg::read("log_level")));
    HttpServer::new(|| 
        App::new()
        .wrap(Logger::default())
        .wrap(middleware::cors())
        .configure(router::init_router)
        // Reference: https://actix.rs/docs/application#application-guards-and-virtual-hosting
    )
    .bind((
        cfg::read("host"), 
        cfg::read("port").parse::<u16>().unwrap(),
    ))?
    // .workers(4) // By default, the number of available physical CPUs is used as the worker count.
    .run()
    .await
}
