use actix_web::{get, web, App, HttpServer, Responder};
use std::env;

#[get("/{id}/{name}/index.html")]
async fn index(web::Path((id, name)): web::Path<(u32, String)>) -> impl Responder {
    format!("Hello CloudRun2 {}! id:{}", name, id)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let port = env::var("PORT").unwrap_or_else(|err|{
        match err {
            env::VarError::NotPresent => "8080".to_string(),
            _ => panic!("unknown error"),
        }
    });
    HttpServer::new(|| App::new().service(index))
        .bind(format!("0.0.0.0:{}",port))?
        .run()
        .await
}