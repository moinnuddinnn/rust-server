//HANDLERS !! DO NOT REMOVE !!

use actix_web::{web, HttpResponse, Responder}; //HTTP RESPONSES
use tera::{Context, Tera};

//INDEX/HOME PAGE

pub async fn index(tmpl: web::Data<Tera>) -> impl Responder {
    let mut ctx = Context::new();
    ctx.insert("name", "Rust Developer");

    match tmpl.render("index.html", &ctx) {
        Ok(html) => HttpResponse::Ok().content_type("text/html").body(html),
        Err(err) => HttpResponse::InternalServerError()
                        .body(format!("Template error: {}", err)),
    }
}

//ABOUT THE APPLICATION

pub async fn about(tmpl: web::Data<Tera>) -> impl Responder {
    let ctx = Context::new();

    match tmpl.render("about.html", &ctx) {
        Ok(html) => HttpResponse::Ok().content_type("text/html").body(html),
        Err(err) => HttpResponse::InternalServerError()
                        .body(format!("Template error: {}", err)),
    }
}




