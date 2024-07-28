use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use config::settings::{EnvVariables, Settings};
use dotenv::dotenv;
pub mod config;

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

async fn manual_hello() -> impl Responder {
    HttpResponse::Ok().body("Hey there!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    HttpServer::new(|| {
        App::new()
            .service(hello)
            .service(echo)
            .route("/hey", web::get().to(manual_hello))
    })
    .bind((
        Settings::expect_env(EnvVariables::HostAddress).as_str(),
        Settings::expect_env(EnvVariables::HostPort)
            .parse::<u16>()
            .expect("HOST_PORT should be a number"),
    ))?
    .run()
    .await
}
