mod db;

use actix_web::{delete, get, post, put, App, HttpResponse, HttpServer, Responder};
use db::establish_connection;
use diesel::mysql::MysqlConnection;

#[get("/")]
async fn get(db: MysqlConnection) -> impl Responder {
    HttpResponse::Ok().body("get ok")
}
#[post("/")]
async fn post(db: MysqlConnection) -> impl Responder {
    HttpResponse::Ok().body("post ok")
}
#[put("/")]
async fn put(db: MysqlConnection) -> impl Responder {
    HttpResponse::Ok().body("put ok")
}
#[delete("/")]
async fn delete(db: MysqlConnection) -> impl Responder {
    HttpResponse::Ok().body("delete ok")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let connection = establish_connection();

    HttpServer::new(|| {
        App::new()
            .app_data(connection.clone())
            .service(get)
            .service(post)
            .service(put)
            .service(delete)
    })
    .bind("127.0.0.1:8088")?
    .run()
    .await
}
