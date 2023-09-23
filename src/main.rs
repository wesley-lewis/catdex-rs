use actix_web::{web, App, HttpServer, Result};
use actix_files::{NamedFile};

async fn index() -> Result<NamedFile> {
    Ok(NamedFile::open("./static/index.html")?)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Listening on port 8080");
    HttpServer::new(|| {
        App::new().route("/", web::get().to(index))
    })
    .bind("localhost:8080")?
    .run()
    .await
}
