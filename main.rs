use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use serde::{Deserialize, Serialize};
use std::io;
mod user;
mod message;

// Structure for JSON serialization/deserialization
#[derive(Serialize, Deserialize)]
struct ApiResponse {
    message: String,
    status: String,
}

// Handler for the root path
async fn index() -> impl Responder {
    HttpResponse::Ok().body("Hello, Actix Web!")
}

// Handler that returns JSON
async fn get_info() -> impl Responder {
    let response = ApiResponse {
        message: String::from("This is a basic web server"),
        status: String::from("success"),
    };
    
    HttpResponse::Ok().json(response)
}

// Health check endpoint
async fn health_check() -> impl Responder {
    HttpResponse::Ok().body("Server is up and running!")
}

// Test endpoint
async fn test() -> impl Responder {
    HttpResponse::Ok().body("This is a test endpoint")
}

// Test async endpoint
async fn test_async() -> impl Responder {
    HttpResponse::Ok().body("This is a test endpoint")
}

#[actix_web::main]
async fn main() -> io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(index))
            .route("/info", web::get().to(get_info))
            .route("/health", web::get().to(health_check))
            .route("/test", web::get().to(test))
            .route("/test_async", web::get().to(test_async))
    })
    .bind("0.0.0.0:8080")?
    .run()
    .await
}

// Note: You'll need to include these dependencies in your Cargo.toml:
// [dependencies]
// actix-web = "4.0"
// serde = { version = "1.0", features = ["derive"] }