use axum::{
    routing::get,
    Router, Json, extract::{Query, Path},
};
use serde::{Deserialize, Serialize};
use std::net::SocketAddr;
use tokio::net::TcpListener;

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
async fn get_users(Query(query): Query<UserFilter>) -> Json<Vec<User>> {
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

    Json(filtered_users)
}

/// Handles GET request to retrieve a specific user by ID
///
/// # Arguments
///
/// * `path` - A Path extractor containing the user ID
///
/// Returns a JSON object of the requested user
async fn get_user(Path(user_id): Path<u32>) -> Json<User> {
    // In a real application, we would fetch the user from a database
    let user = User { id: user_id, name: format!("User {}", user_id) };
    Json(user)
}

/// Handles POST request to create a new user
///
/// # Arguments
///
/// * `user` - JSON payload representing the new user
///
/// Returns the created user
async fn create_user(Json(user): Json<User>) -> Json<User> {
    // In a real application, we would save the user to a database
    Json(user)
}

/// Starts the web server and configures the routes
#[tokio::main]
pub async fn main() {
    const HOST: &str = "127.0.0.1";
    const PORT: u16 = 8080;
    // Run to get all users http://127.0.0.1:8080/api/v1/users
    // Run to get user with id 1 http://127.0.0.1:8080/api/v1/users/1
    // Run to get filter user http://127.0.0.1:8080/api/v1/users?name=Alice
    println!("Starting web server at http://{}:{}", HOST, PORT);
    
    let app = Router::new()
        .route("/api/v1/users", get(get_users).post(create_user))
        .route("/api/v1/users/:id", get(get_user));

    let addr = SocketAddr::from(([127, 0, 0, 1], PORT));
    let listener = TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}