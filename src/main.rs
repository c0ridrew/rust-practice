use actix_web::{delete, get, post, put, web, App, HttpResponse, HttpServer, Responder, Result};
use diesel::prelude::*;
use hello_rust::db::establish_connection;
use hello_rust::model::{NewUser, User};
use hello_rust::schema::users;

#[get("/")]
async fn get() -> Result<impl Responder> {
    let connection = establish_connection();
    let users = users::table.load::<User>(&connection).expect("error");
    Ok(web::Json(users))
}

#[post("/")]
async fn post(item: web::Json<NewUser>) -> impl Responder {
    let connection = establish_connection();
    let new_user = NewUser {
        email: item.email.to_string(),
    };
    diesel::insert_into(users::dsl::users)
        .values(&new_user)
        .execute(&connection)
        .expect("Error saving new post");
    HttpResponse::Created().body("get ok")
}

#[put("/")]
async fn put() -> impl Responder {
    HttpResponse::Created().body("get ok")
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
