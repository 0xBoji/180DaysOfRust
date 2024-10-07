mod web_development_actix_swaggerui;
mod web_development_axum_swaggerui;

pub fn run() {
    println!("Days 19-21: Web Development with Swagger UI");
    // let _ = web_development_actix_swaggerui::main();
    let _ = web_development_axum_swaggerui::main();
}