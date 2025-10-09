use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use tera::{Tera, Context};

async fn index(tmpl: web::Data<Tera>) -> impl Responder {
    let mut ctx = Context::new();
    ctx.insert("title", "Home");
    let rendered = tmpl.render("index.html", &ctx).unwrap();
    HttpResponse::Ok().content_type("text/html").body(rendered)
}

async fn about(tmpl: web::Data<Tera>) -> impl Responder {
    let mut ctx = Context::new();
    ctx.insert("title", "About");
    let rendered = tmpl.render("about.html", &ctx).unwrap();
    HttpResponse::Ok().content_type("text/html").body(rendered)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let tera = Tera::new("templates/**/*").unwrap();

    println!("🚀 Server running at http://127.0.0.1:8080/");
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(tera.clone()))
            .route("/", web::get().to(index))
            .route("/about", web::get().to(about))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
