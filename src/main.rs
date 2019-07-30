extern crate actix_web;

use std::{io};
use actix_web::{middleware, web, App, HttpServer};

fn index() -> &'static str {
    "I am alive."
}

fn main() -> io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .wrap(middleware::Logger::default())
            .route("/", web::get().to(index))
    })
    .bind("0.0.0.0:3001")?
    .run()
}
