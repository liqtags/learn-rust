// Web Development in Rust
// This example demonstrates how to build a RESTful API using Actix-web

use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use serde::{Deserialize, Serialize};
use std::sync::Mutex;

// Define our data models
#[derive(Serialize, Deserialize, Clone)]
struct Todo {
    id: u32,
    title: String,
    completed: bool,
}

// Define our application state
struct AppState {
    todos: Mutex<Vec<Todo>>,
}

// Handler for GET /todos
async fn get_todos(data: web::Data<AppState>) -> impl Responder {
    let todos = data.todos.lock().unwrap();
    HttpResponse::Ok().json(todos.clone())
}

// Handler for GET /todos/{id}
async fn get_todo(
    data: web::Data<AppState>,
    id: web::Path<u32>,
) -> impl Responder {
    let todos = data.todos.lock().unwrap();
    if let Some(todo) = todos.iter().find(|t| t.id == *id) {
        HttpResponse::Ok().json(todo.clone())
    } else {
        HttpResponse::NotFound().finish()
    }
}

// Handler for POST /todos
async fn create_todo(
    data: web::Data<AppState>,
    todo: web::Json<Todo>,
) -> impl Responder {
    let mut todos = data.todos.lock().unwrap();
    todos.push(todo.into_inner());
    HttpResponse::Created().finish()
}

// Handler for PUT /todos/{id}
async fn update_todo(
    data: web::Data<AppState>,
    id: web::Path<u32>,
    todo: web::Json<Todo>,
) -> impl Responder {
    let mut todos = data.todos.lock().unwrap();
    if let Some(t) = todos.iter_mut().find(|t| t.id == *id) {
        *t = todo.into_inner();
        HttpResponse::Ok().finish()
    } else {
        HttpResponse::NotFound().finish()
    }
}

// Handler for DELETE /todos/{id}
async fn delete_todo(
    data: web::Data<AppState>,
    id: web::Path<u32>,
) -> impl Responder {
    let mut todos = data.todos.lock().unwrap();
    if let Some(pos) = todos.iter().position(|t| t.id == *id) {
        todos.remove(pos);
        HttpResponse::NoContent().finish()
    } else {
        HttpResponse::NotFound().finish()
    }
}

// Middleware for logging requests
async fn logger_middleware(
    req: actix_web::HttpRequest,
    next: actix_web::dev::ServiceRequest,
) -> Result<actix_web::dev::ServiceResponse, actix_web::Error> {
    println!("Request: {} {}", req.method(), req.uri());
    next.call(req).await
}

// Error handler
async fn error_handler(err: actix_web::error::Error) -> impl Responder {
    HttpResponse::InternalServerError().body(format!("Error: {}", err))
}

// Main function
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Initialize application state
    let app_state = web::Data::new(AppState {
        todos: Mutex::new(vec![
            Todo {
                id: 1,
                title: "Learn Rust".to_string(),
                completed: false,
            },
            Todo {
                id: 2,
                title: "Build a web app".to_string(),
                completed: false,
            },
        ]),
    });

    // Start the server
    HttpServer::new(move || {
        App::new()
            .app_data(app_state.clone())
            .wrap(actix_web::middleware::Logger::default())
            .wrap(actix_web::middleware::Compress::default())
            .service(
                web::scope("/api")
                    .service(
                        web::resource("/todos")
                            .route(web::get().to(get_todos))
                            .route(web::post().to(create_todo)),
                    )
                    .service(
                        web::resource("/todos/{id}")
                            .route(web::get().to(get_todo))
                            .route(web::put().to(update_todo))
                            .route(web::delete().to(delete_todo)),
                    ),
            )
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}

// Example of how to test the API
#[cfg(test)]
mod tests {
    use super::*;
    use actix_web::test;

    #[actix_web::test]
    async fn test_get_todos() {
        let app_state = web::Data::new(AppState {
            todos: Mutex::new(vec![
                Todo {
                    id: 1,
                    title: "Test".to_string(),
                    completed: false,
                },
            ]),
        });

        let app = test::init_service(
            App::new()
                .app_data(app_state)
                .service(web::resource("/todos").route(web::get().to(get_todos))),
        )
        .await;

        let req = test::TestRequest::get().uri("/todos").to_request();
        let resp = test::call_service(&app, req).await;
        assert!(resp.status().is_success());
    }
} 