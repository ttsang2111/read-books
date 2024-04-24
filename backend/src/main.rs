use actix_web::{web, App, HttpResponse,HttpServer, Responder};

async fn list_books() -> impl Responder {
    HttpResponse::Ok().body("List of books")
}
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new().route("/books", web::get().to(list_books))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}