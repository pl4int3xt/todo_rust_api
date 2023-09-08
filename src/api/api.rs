use actix_web::web;
use actix_web::{web::{
    Data,
    Json,
}, post, HttpResponse};
use crate::{models::todo::Todo, repository::database::Database};

#[post("/todos")]
pub async fn create_todo(db: Data<Database>, new_todo: Json<Todo>) -> HttpResponse {
    let todo = db.create_todo(new_todo.into_inner());
    match todo {
        Ok(todo) => HttpResponse::Ok().json(todo),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string())
    }
}

#[get("/todos")]
pub async fn get_todos(db: Data<Database>) -> HttpResponse {
    let todos = db.get_todos();
    HttpResponse::Ok().json(todos)

}

pub fn config(cfg: &mut web::ServiceConfig){
    cfg.service(
        web::scope("/api")
            .service(create_todo)
    );
}