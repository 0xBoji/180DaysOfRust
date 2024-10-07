use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use serde::{Deserialize, Serialize};

/// Represents a user in the system
#[derive(Serialize, Deserialize)]
struct User {
    id: u32,
    name: String,
}

/// Represents query parameters for filtering users
#[derive(Deserialize)]
struct UserFilter {
    name: Option<String>,
}

/// Handles GET request to retrieve all users
///
/// # Arguments
///
/// * `query` - Query parameters for filtering users
///
/// Returns a JSON array of filtered users in the system
async fn get_users(query: web::Query<UserFilter>) -> impl Responder {
    let users = vec![
        User { id: 1, name: "Alice".to_string() },
        User { id: 2, name: "Bob".to_string() },
        User { id: 3, name: "Charlie".to_string() },
        User { id: 4, name: "Bobby Pickle".to_string() },
    ];

    let filtered_users: Vec<User> = users.into_iter()
        .filter(|user| {
            query.name.as_ref().map_or(true, |name| user.name.to_lowercase().contains(&name.to_lowercase()))
        })
        .collect();

    HttpResponse::Ok().json(filtered_users)
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
    // Run to get all users http://127.0.0.1:8080/api/v1/users
    // Run to get user with id 1 http://127.0.0.1:8080/api/v1/users/1
    // Run to get filter user http://127.0.0.1:8080/api/v1/users
    println!("Starting web server at http://{}:{}", HOST, PORT);
    
    HttpServer::new(|| {
        App::new()
            .service(
                web::scope("/api/v1")
                    .route("/users", web::get().to(get_users))
                    .route("/users/{id}", web::get().to(get_user))
                    .route("/users", web::post().to(create_user))
            )
    })
    .bind((HOST, PORT))?
    .run()
    .await
}