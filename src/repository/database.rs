use std::fmt::Error;
use chrono::prelude::*;
use std::sync::{Arc, Mutex};

use crate::models::todo::Todo;

pub struct Database{
    pub todos: Arc<Mutex<Vec<Todo>>>,
}

impl Database {
    pub fn new() -> Self {
        let todos = Arc::new(Mutex::new(vec![]));
        Database { todos }
    }

    pub fn crete_todo(&self, todo:Todo) -> Result<Todo, Error>{
        let mut todos = self.todos.lock().unwrap();
        let id = uuid::Uuid::new_v4();
        let created_at = Utc::now();
        let updated_at = Utc::now();
        let todo = Todo{
            id: Some(id),
            created_at: Some(created_at),
            updated_at: Some(updated_at),
            ..todo
        };
        todo.push(todo.clone());
        Ok(todo)
    }
}