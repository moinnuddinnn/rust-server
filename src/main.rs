mod handlers;
mod routes;
mod models;

use actix_files as fs;
use actix_web::{App, HttpServer, web};
use tera::Tera;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // initialize tera (template engine)
    let tera = Tera::new("templates/**/*").expect("Error loading templates");

    println!("🚀 Server running at http://127.0.0.1:8080");

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(tera.clone())) // share tera with handlers
            .service(fs::Files::new("/static", "static").show_files_listing()) // serve static files
            .configure(routes::init) // load all routes
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
