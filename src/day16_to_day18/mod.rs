mod web_development_actix;
mod web_development_axum;

pub fn run() {
    println!("Days 16-18: Web Development");
    // let _ = web_development_actix::main();
    let _ = web_development_axum::main();
}