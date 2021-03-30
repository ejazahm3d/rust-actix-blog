mod models;
mod routes;

use actix_web::{dev::Server, web, App, HttpServer};
use routes::greet::greet;
use routes::health_check::health_check;
use std::net::TcpListener;
use web::get;

pub fn run(listener: TcpListener) -> Result<Server, std::io::Error> {
    let server = HttpServer::new(|| {
        App::new()
            .route("/", get().to(greet))
            .route("/health_check", get().to(health_check))
            .route("/{name}", get().to(greet))
    })
    .listen(listener)?
    .run();
    Ok(server)
}
