use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use serde::{Deserialize, Serialize};

/// Represents a user in the system
#[derive(Serialize, Deserialize)]
struct User {
    id: u32,
    name: String,
}

/// Handles GET request to retrieve all users
///
/// Returns a JSON array of all users in the system
async fn get_users() -> impl Responder {
    let users = vec![
        User { id: 1, name: "Alice".to_string() },
        User { id: 2, name: "Bob".to_string() },
    ];
    HttpResponse::Ok().json(users)
}

/// Handles GET request to retrieve a specific user by ID
///
/// # Arguments
///
/// * `path` - A Path extractor containing the user ID
///
/// Returns a JSON object of the requested user or a 404 if not found
async fn get_user(path: web::Path<u32>) -> impl Responder {
    let user_id = path.into_inner();
    // In a real application, we would fetch the user from a database
    let user = User { id: user_id, name: format!("User {}", user_id) };
    HttpResponse::Ok().json(user)
}

/// Handles POST request to create a new user
///
/// # Arguments
///
/// * `user` - JSON payload representing the new user
///
/// Returns the created user with a 201 status code
async fn create_user(user: web::Json<User>) -> impl Responder {
    // In a real application, we would save the user to a database
    HttpResponse::Created().json(user.into_inner())
}

/// Starts the web server and configures the routes
#[actix_web::main]
pub async fn main() -> std::io::Result<()> {
    const HOST: &str = "127.0.0.1";
    const PORT: u16 = 8080;

    println!("Starting web server at http://{}:{}", HOST, PORT);
    
    HttpServer::new(|| {
        App::new()
            .route("/users", web::get().to(get_users))
            .route("/users/{id}", web::get().to(get_user))
            .route("/users", web::post().to(create_user))
    })
    .bind((HOST, PORT))?
    .run()
    .await
}