use actix_web::web::Data;
use actix_web::{delete, get, post, put, web, App, HttpResponse, HttpServer, Responder, Result};
use diesel::prelude::*;
use hello_rust::db::{establish_connection, Pool};
use hello_rust::model::{NewUser, User};
use hello_rust::schema::users;

#[get("/users")]
async fn get(db: web::Data<Pool>) -> Result<impl Responder> {
    let conn = db.get().unwrap();
    let users = users::table.load::<User>(&conn).expect("error");
    Ok(web::Json(users))
}

#[get("/users/{id}")]
async fn find_by_id(db: web::Data<Pool>, path: web::Path<i32>) -> Result<impl Responder> {
    let conn = db.get().unwrap();
    let id = path.into_inner();
    let user = users::table
        .filter(users::id.eq(id))
        .load::<User>(&conn)
        .expect("error");

    Ok(web::Json(user))
}

#[get("/users/search")]
async fn search(db: web::Data<Pool>, query: web::Query<User>) -> Result<impl Responder> {
    let conn = db.get().unwrap();
    let users = users::table.load::<User>(&conn).expect("error");
    Ok(web::Json(users))
}

#[post("/users")]
async fn post(db: web::Data<Pool>, item: web::Json<NewUser>) -> impl Responder {
    let conn = db.get().unwrap();
    let new_user = NewUser {
        email: item.email.to_string(),
    };
    diesel::insert_into(users::dsl::users)
        .values(&new_user)
        .execute(&conn)
        .expect("Error saving new post");
    HttpResponse::Created().body("get ok")
}

#[put("/users")]
async fn put(db: web::Data<Pool>, item: web::Json<NewUser>) -> impl Responder {
    let conn = db.get().unwrap();
    let new_user = NewUser {
        email: item.email.to_string(),
    };
    diesel::insert_into(users::dsl::users)
        .values(&new_user)
        .execute(&conn)
        .expect("Error saving new post");
    HttpResponse::Created().body("get ok")
}

#[delete("/")]
async fn delete(db: web::Data<Pool>) -> impl Responder {
    let conn = db.get().unwrap();
    HttpResponse::Ok().body("delete ok")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let pool = establish_connection();

    HttpServer::new(move || {
        App::new()
            .app_data(Data::new(pool.clone()))
            .service(get)
            .service(find_by_id)
            .service(search)
            .service(post)
            .service(put)
            .service(delete)
    })
    .bind("127.0.0.1:8088")?
    .run()
    .await
}
