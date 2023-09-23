use actix_web::{web, App, HttpResponse, HttpServer, Responder, get};

#[get("/hello")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("hello world")
}

struct AppState {
    app_name: String,
}

#[get("/")]
async fn index(data: web::Data<AppState>) -> String {
    let app_name = &data.app_name;
    format!("Hello {app_name}!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Listening on port 8080");
    HttpServer::new(|| {
        App::new().app_data(web::Data::new(AppState {
            app_name: String::from("Actix Web"),
        }))
            .service(index)
            .service(hello)
    })
    .bind("localhost:8080")?
    .run()
    .await
}
