use actix_web::middleware::Logger;
use actix_web::{get, App, HttpResponse, HttpServer, Responder, Result, web};
use serde::Serialize;

mod api;
mod models;
mod repository;


#[derive(Serialize)]
pub struct GenericResponse{
    pub status: String,
    pub message: String
}

#[get("/api/health")]
async fn health_checker_handler() -> impl Responder {
    const MESSAGE: &str = "CRUD TODO API WITH RUST AND ACTIX WEB";

    let response_json = GenericResponse {
        status: "Success".to_string(),
        message: MESSAGE.to_string()
    };

    HttpResponse::Ok().json(response_json)
}

async fn not_found() -> Result<HttpResponse>{
    const NOT_FOUND_MESSAGE: &str = "Resource not found";
    let response_json = GenericResponse {
        status: "Success".to_string(),
        message: NOT_FOUND_MESSAGE.to_string()
    };
    Ok(HttpResponse::NotFound().json(response_json))
}

#[actix_web::main]
async fn main() -> std::io::Result<()>{
    let todo_db = repository::database::Database::new();
    let app_data = web::Data::new(todo_db);

    if std::env::var_os("RUST_LOG").is_none(){
        std::env::set_var("RUST_LOG", "actix_web=info");
    }
    env_logger::init();

    println!("Server started successfully");

    HttpServer::new( move || {
        App::new()
            .app_data(app_data.clone())
            .configure(api::api::config)
            .service(health_checker_handler)
            .default_service(web::route().to(not_found))
            .wrap(Logger::default())
    })
    .bind(("127.0.0.1", 8000))?
    .run()
    .await
}
