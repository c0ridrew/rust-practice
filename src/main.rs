pub mod db;
pub mod model;
pub mod schema;

use actix_web::{delete, get, post, put, App, HttpResponse, HttpServer, Responder};
use db::establish_connection;
use diesel::deserialize::QueryableByName;
use diesel::mysql::MysqlConnection;
use diesel::prelude::*;
use diesel::sql_query;

#[get("/")]
async fn get(db: MysqlConnection) -> impl Responder {
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
