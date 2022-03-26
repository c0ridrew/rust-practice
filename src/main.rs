use actix_web::{web, App, HttpResponse, HttpServer, Responder};

async fn root() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

async fn index2() -> impl Responder {
    HttpResponse::Ok().json("{\"message\":\"Hello world again!\"}")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(root))
            .route("/again", web::get().to(index2))
    })
    .bind("127.0.0.1:8088")?
    .run()
    .await
}