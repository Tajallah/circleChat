use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use serde::{Deserialize, Serialize};
use std::io;

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

#[actix_web::main]
async fn main() -> io::Result<()> {
    println!("Starting server at http://127.0.0.1:8080");
    
    HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(index))
            .route("/info", web::get().to(get_info))
            .route("/health", web::get().to(health_check))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}

// Note: You'll need to include these dependencies in your Cargo.toml:
// [dependencies]
// actix-web = "4.0"
// serde = { version = "1.0", features = ["derive"] }