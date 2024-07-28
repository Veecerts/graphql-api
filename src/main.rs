use actix_cors::Cors;
use actix_web::{guard, web, App, HttpResponse, HttpServer, Result};
use async_graphql::http::{graphiql_plugin_explorer, GraphiQLSource};
use async_graphql_actix_web::GraphQL;
use config::{
    database::DB,
    schema::build_schema,
    settings::{EnvVariables, Settings},
};
use dotenv::dotenv;
pub mod config;
pub mod files;
pub mod services;

async fn index_graphiql() -> Result<HttpResponse> {
    Ok(HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(
            GraphiQLSource::build()
                .plugins(&[graphiql_plugin_explorer()])
                .endpoint("/")
                .finish(),
        ))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .with_test_writer()
        .init();
    let conn = DB::connect().await.expect("Database connection failed");
    HttpServer::new(move || {
        let schema = build_schema(conn.clone());
        let cors = Cors::permissive();
        App::new()
            .wrap(cors)
            .service(
                web::resource("/")
                    .guard(guard::Post())
                    .to(GraphQL::new(schema)),
            )
            .service(web::resource("/").guard(guard::Get()).to(index_graphiql))
    })
    .bind((
        Settings::expect_env(EnvVariables::HostAddress).as_str(),
        Settings::expect_env(EnvVariables::Port)
            .parse::<u16>()
            .expect("HOST_PORT should be a number"),
    ))?
    .run()
    .await
}
