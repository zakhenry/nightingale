use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use std::env;

fn health_check() -> impl Responder {
    println!("healthcheck");
    HttpResponse::Ok().body("I'm healthy!")
}

fn get_server_port() -> u16 {
    env::var("PORT")
        .ok()
        .and_then(|p| p.parse().ok())
        .unwrap_or(8181)
}

fn main() {

    HttpServer::new(move || {
        App::new()
            .route("/healthcheck", web::get().to(health_check))
    })
    .bind(("0.0.0.0", get_server_port()))
    .unwrap()
    .run()
    .unwrap();
}
