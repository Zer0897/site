#[cfg(test)]
mod tests;

use tera;
use std::io;
use std::path::{Path, PathBuf};
use serde::Deserialize;
use actix_files as fs;
use actix_web::{
    HttpRequest, HttpResponse,
    Error, HttpServer,
    Responder, Result,
    web, middleware,
    error, App
};


#[derive(Deserialize)]
struct File {
    path: PathBuf
}


fn index(tmpl: web::Data<tera::Tera>) -> Result<HttpResponse, Error> {
    Ok(HttpResponse::Ok().content_type("text/html").body(
        tmpl.render("index.html", &tera::Context::new())
            .map_err(|_| error::ErrorInternalServerError("Template error."))?
    ))
}


fn page((tmpl, pg): (web::Data<tera::Tera>, web::Path<File>)) -> Result<HttpResponse, Error> {
    Ok(HttpResponse::Ok().content_type("text/html").body(
        tmpl.render(
            pg.path.with_extension("html").to_str().unwrap(),
            &tera::Context::new()
        ).map_err(|_| error::ErrorInternalServerError("Template error."))?
    ))
}


fn asset(file: web::Path<File>) -> io::Result<fs::NamedFile> {
    Ok(fs::NamedFile::open(
        Path::new("./src/static/assets").join(&file.path))?
    )
}


fn main() {
    std::env::set_var("RUST_LOG", "actix_web=debug");
    env_logger::init();

    HttpServer::new(|| {
        let tera = tera::compile_templates!("./src/templates/**/*");

        App::new()
            .data(tera)
            .wrap(middleware::Logger::default())
            .route("/", web::get().to(index))
            .route("/{path}", web::get().to(page))
            .route("/s/{path:.*}", web::get().to(asset))
    })
        .bind("127.0.0.1:8080")
        .unwrap()
        .workers(1)
        .run()
        .unwrap()
}
