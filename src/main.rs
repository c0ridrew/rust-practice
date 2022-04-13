#[macro_use]
extern crate diesel;
use diesel::ExpressionMethods;
use diesel::QueryDsl;
use diesel::RunQueryDsl;

use actix_web::web::Data;
use actix_web::{delete, get, post, put, web, App, HttpResponse, HttpServer, Responder, Result};
mod db;
mod model;
mod schema;

#[get("/users/{id}")]
async fn get(db: web::Data<db::Pool>, path: web::Path<i32>) -> Result<impl Responder> {
    let conn = db.get().unwrap();
    let id = path.into_inner();
    let user = schema::users::table
        .select(schema::users::email)
        .filter(schema::users::id.eq(id))
        .load::<String>(&conn)
        .expect("error");

    Ok(web::Json(user))
}

#[post("/users")]
async fn post(db: web::Data<db::Pool>, item: web::Json<model::User>) -> Result<impl Responder> {
    let conn = db.get().unwrap();
    let new_user = model::User {
        email: item.email.to_string(),
    };
    diesel::insert_into(schema::users::dsl::users)
        .values(&new_user)
        .execute(&conn)
        .expect("Error saving new post");

    Ok(HttpResponse::Created().body("post ok"))
}

#[put("/users/{id}")]
async fn put(
    db: web::Data<db::Pool>,
    path: web::Path<i32>,
    item: web::Json<model::User>,
) -> Result<impl Responder> {
    let id = path.into_inner();
    let conn = db.get().unwrap();
    let target = schema::users::dsl::users.filter(schema::users::dsl::id.eq(id));

    diesel::update(target)
        .set(schema::users::dsl::email.eq(item.email.to_string()))
        .execute(&conn)
        .expect("Error updating new post");

    Ok(HttpResponse::Created().body("update ok"))
}

#[delete("/users/{id}")]
async fn destroy(db: web::Data<db::Pool>, path: web::Path<i32>) -> Result<impl Responder> {
    let id = path.into_inner();
    let conn = db.get().unwrap();
    let target = schema::users::dsl::users.filter(schema::users::dsl::id.eq(id));

    diesel::delete(target)
        .execute(&conn)
        .expect("Error deleting new post");

    Ok(HttpResponse::Created().body("Delete ok"))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // db moduleからestablish_connection関数をimport
    let pool = db::establish_connection();

    // app_dataを用いactix_webにdb poolをinject
    HttpServer::new(move || {
        App::new()
            .app_data(Data::new(pool.clone()))
            .service(get)
            .service(post)
            .service(put)
            .service(destroy)
    })
    .bind("127.0.0.1:8088")?
    .run()
    .await
}
