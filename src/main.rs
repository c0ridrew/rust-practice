use actix_web::{delete, get, post, put, App, HttpResponse, HttpServer, Responder};

#[get("/")]
async fn get() -> impl Responder {
    HttpResponse::Ok().body("get ok")
}
#[post("/")]
async fn post() -> impl Responder {
    HttpResponse::Ok().body("post ok")
}
#[put("/")]
async fn put() -> impl Responder {
    HttpResponse::Ok().body("put ok")
}
#[delete("/")]
async fn delete() -> impl Responder {
    HttpResponse::Ok().body("delete ok")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(get)
            .service(post)
            .service(put)
            .service(delete)
    })
    .bind("127.0.0.1:8088")?
    .run()
    .await
}
