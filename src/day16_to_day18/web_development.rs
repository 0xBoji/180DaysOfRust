use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use serde::{Deserialize, Serialize};

// định nghĩa user
#[derive(Serialize, Deserialize)]
struct User {
    id: u32,
    name: String,
}

// hàm req GET để lấy danh sách user
async fn get_users() -> impl Responder {
    let users = vec![
        User { id: 1, name: "Alice".to_string() },
        User { id: 2, name: "Bob".to_string() },
    ];
    HttpResponse::Ok().json(users)
}

// hàm req GET để get thông tin một detail của user
async fn get_user(path: web::Path<u32>) -> impl Responder {
    let user_id = path.into_inner();
    let user = User { id: user_id, name: format!("User {}", user_id) };
    HttpResponse::Ok().json(user)
}

// hàm req POST để tạo user mới
async fn create_user(user: web::Json<User>) -> impl Responder {
    HttpResponse::Created().json(user.into_inner())
}

// server
#[actix_web::main]
pub async fn main() -> std::io::Result<()> {
    println!("Deploy web server at http://127.0.0.1:8080");
    
    HttpServer::new(|| {
        App::new()
            .route("/users", web::get().to(get_users))
            .route("/users/{id}", web::get().to(get_user))
            .route("/users", web::post().to(create_user))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}