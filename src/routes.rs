//ROUTING

use actix_web::web;
use crate::handlers;

pub fn init(cfg: &mut web::ServiceConfig) {
    cfg.route("/", web::get().to(handlers::index))
       .route("/about", web::get().to(handlers::about));
}

