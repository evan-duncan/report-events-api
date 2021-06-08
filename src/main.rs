use actix_web::middleware::Logger;
use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use env_logger::Env;
use influx_db_client::Client;

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello World!")
}

#[get("/health_check")]
async fn health_check() -> impl Responder {
    let client = Client::default().set_authentication("root", "root");
    match client.ping().await {
        true => HttpResponse::Ok(),
        _ => HttpResponse::ServiceUnavailable(),
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();

    HttpServer::new(|| {
        App::new()
            .wrap(Logger::default())
            .service(hello)
            .service(health_check)
    })
    .bind("127.0.0.1:1337")?
    .run()
    .await
}
