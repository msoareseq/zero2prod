use actix_web::dev::Server;
use actix_web::{App, HttpResponse, HttpServer, Responder, web};
use std::net::TcpListener;

async fn health_check() -> impl Responder {
    HttpResponse::Ok()
}

pub fn run(listener: TcpListener) -> Result<Server, std::io::Error> {
    let server = HttpServer::new(|| App::new().route("/health_check", web::get().to(health_check)))
        .listen(listener)?
        .run();

    Ok(server)
}
