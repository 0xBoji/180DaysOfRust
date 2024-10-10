use axum::{
    routing::get,
    Router, Json, extract::{Query, Path, State},
};
use serde::{Deserialize, Serialize};
use std::net::SocketAddr;
use tokio::net::TcpListener;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;
use deadpool_postgres::{Config, Pool, Runtime};
use tokio_postgres::NoTls;

/// Represents a user in the system
#[derive(Serialize, Deserialize, utoipa::ToSchema)]
struct User {
    id: i32,
    name: String,
}

/// Represents query parameters for filtering users
#[derive(Deserialize, utoipa::IntoParams, utoipa::ToSchema)]
struct UserFilter {
    name: Option<String>,
}

#[derive(Clone)]
struct AppState {
    db: Pool,
}

/// Handles GET request to retrieve all users
///
/// Returns a JSON array of filtered users in the system
#[utoipa::path(
    get,
    path = "/api/v1/users",
    params(UserFilter),
    responses(
        (status = 200, description = "List of users", body = Vec<User>)
    )
)]
async fn get_users(
    State(state): State<AppState>,
    Query(query): Query<UserFilter>
) -> Json<Vec<User>> {
    let conn = state.db.get().await.unwrap();
    let rows = conn.query("SELECT id, name FROM users", &[]).await.unwrap();

    let mut users: Vec<User> = rows.iter().map(|row| User {
        id: row.get(0),
        name: row.get(1),
    }).collect();

    if let Some(name_filter) = query.name {
        users.retain(|user| user.name.to_lowercase().contains(&name_filter.to_lowercase()));
    }

    Json(users)
}

/// Handles GET request to retrieve a specific user by ID
///
/// Returns a JSON object of the requested user, or None if not found
#[utoipa::path(
    get,
    path = "/api/v1/users/{id}",
    params(
        ("id" = i32, Path, description = "User ID")
    ),
    responses(
        (status = 200, description = "User found", body = User),
        (status = 404, description = "User not found")
    )
)]
async fn get_user(
    State(state): State<AppState>,
    Path(user_id): Path<i32>
) -> Json<Option<User>> {
    let conn = state.db.get().await.unwrap();
    let row = conn.query_opt("SELECT id, name FROM users WHERE id = $1", &[&user_id]).await.unwrap();

    let user = row.map(|row| User {
        id: row.get(0),
        name: row.get(1),
    });

    Json(user)
}

/// Handles POST request to create a new user
///
/// Returns the created user
#[utoipa::path(
    post,
    path = "/api/v1/users",
    request_body = User,
    responses(
        (status = 201, description = "User created", body = User)
    )
)]
async fn create_user(
    State(state): State<AppState>,
    Json(user): Json<User>
) -> Json<User> {
    let conn = state.db.get().await.unwrap();
    let row = conn.query_one(
        "INSERT INTO users (name) VALUES ($1) RETURNING id, name",
        &[&user.name]
    ).await.unwrap();

    let created_user = User {
        id: row.get(0),
        name: row.get(1),
    };

    Json(created_user)
}

#[derive(OpenApi)]
#[openapi(
    paths(get_users, get_user, create_user),
    components(schemas(User, UserFilter))
)]
struct ApiDoc;

async fn test_db_connection(pool: &Pool) -> Result<(), Box<dyn std::error::Error>> {
    let conn = pool.get().await?;
    conn.execute("SELECT 1", &[]).await?;
    println!("Database connection successful!");
    Ok(())
}

/// Starts the web server and configures the routes
#[tokio::main]
pub async fn main() -> Result<(), Box<dyn std::error::Error>> {
    const HOST: &str = "127.0.0.1";
    const PORT: u16 = 8080;

    // Configure the database connection
    let mut cfg = Config::new();
    cfg.host = Some("localhost".to_string());
    cfg.dbname = Some("axum_users".to_string());
    cfg.user = Some("axum_user".to_string());
    cfg.password = Some("123123".to_string());

    let pool = cfg.create_pool(Some(Runtime::Tokio1), NoTls)?;

    // Test the database connection
    test_db_connection(&pool).await?;
    println!("Database connection test passed.");

    // Create the users table if it doesn't exist
    let conn = pool.get().await?;
    conn.execute(
        "CREATE TABLE IF NOT EXISTS users (
            id SERIAL PRIMARY KEY,
            name TEXT NOT NULL
        )",
        &[]
    ).await?;

    let app_state = AppState { db: pool };

    let app = Router::new()
        .route("/api/v1/users", get(get_users).post(create_user))
        .route("/api/v1/users/:id", get(get_user))
        .merge(SwaggerUi::new("/swagger-ui")
            .url("/api-docs/openapi.json", ApiDoc::openapi()))
        .with_state(app_state);

    let addr = SocketAddr::from(([127, 0, 0, 1], PORT));
    let listener = TcpListener::bind(addr).await?;
    println!("Starting web server at http://{}:{}", HOST, PORT);
    println!("Access Swagger UI at http://{}:{}/swagger-ui/", HOST, PORT);
    axum::serve(listener, app).await?;

    Ok(())
}