use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use serde::{Deserialize, Serialize};

#[derive(Serialize)]
struct HealthResponse {
    status: String,
}

#[derive(Serialize)]
struct User {
    id: u32,
    name: String,
}

#[derive(Deserialize)]
struct CreateUserRequest {
    name: String,
}

// GET /health
async fn health() -> impl Responder {
    HttpResponse::Ok().json(HealthResponse {
        status: "ok".to_string(),
    })
}

// GET /users
async fn get_users() -> impl Responder {
    let users = vec![
        User { id: 1, name: "Alice".to_string() },
        User { id: 2, name: "Bob".to_string() },
    ];
    HttpResponse::Ok().json(users)
}

// GET /users/{id}
async fn get_user(path: web::Path<u32>) -> impl Responder {
    let id = path.into_inner();
    let user = User {
        id,
        name: format!("User {}", id),
    };
    HttpResponse::Ok().json(user)
}

// POST /users
async fn create_user(body: web::Json<CreateUserRequest>) -> impl Responder {
    let user = User {
        id: 42,
        name: body.name.clone(),
    };
    HttpResponse::Created().json(user)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Server running at http://127.0.0.1:8080");

    HttpServer::new(|| {
        App::new()
            .route("/health", web::get().to(health))
            .service(
                web::scope("/users")
                    .route("", web::get().to(get_users))
                    .route("/{id}", web::get().to(get_user))
                    .route("", web::post().to(create_user)),
            )
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
