extern crate env_logger;
mod controllers;
mod constant;
mod middleware;
use actix_cors::Cors;
use actix_web::{guard, middleware as mid, web, App, HttpRequest, HttpResponse, HttpServer};
use async_graphql::http::{playground_source, GraphQLPlaygroundConfig, MultipartOptions};
use async_graphql::{EmptySubscription, Schema};
use async_graphql_actix_web::{Request, Response};
use controllers::{MutationRoot, MyToken, QueryRoot, UserSchema};
use std::env;

async fn index(schema: web::Data<UserSchema>, req: HttpRequest, gql_request: Request) -> Response {
    let token = req
        .headers()
        .get("Authorization")
        .and_then(|value| value.to_str().map(|s| MyToken(s.to_string())).ok());
    let mut request = gql_request.into_inner();
    if let Some(token) = token {
        request = request.data(token);
    }
    schema.execute(request).await.into()
}

async fn gql_playgound() -> HttpResponse {
    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(playground_source(GraphQLPlaygroundConfig::new("/")))
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();
    let schema = Schema::build(QueryRoot, MutationRoot, EmptySubscription).finish();
    println!("🚀 Playground: http://localhost:6000");
    let port: u16 = env::var("PORT")
        .unwrap_or_else(|_| "6000".to_string())
        .parse()
        .expect("PORT must be a number");

    HttpServer::new(move || {
        App::new()
            .wrap(mid::Logger::default())
            .wrap(
                Cors::default()
                    .allow_any_origin()
                    .allow_any_header()
                    .allow_any_method()
                    .supports_credentials(),
            )
            .data(schema.clone())
            .service(
                web::resource("/")
                    .guard(guard::Post())
                    .to(index)
                    .app_data(MultipartOptions::default().max_num_files(3)),
            ).service(web::resource("/").guard(guard::Get()).to(gql_playgound))
    })
    .bind(("0.0.0.0".to_string(), port))?
    .run()
    .await
}
