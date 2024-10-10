pub mod web_development_axum_swaggerui_postgresql;

pub fn run() {
    println!("Days 22 - 24: Web Development with Swagger UI & Connect to Postgres");
    if let Err(e) = web_development_axum_swaggerui_postgresql::main() {
        eprintln!("Error: {}", e);
    }
}