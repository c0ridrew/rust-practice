use diesel::Insertable;
use diesel::Queryable;
mod schema;

#[derive(diesel::Queryable)]
pub struct User {
    pub id: i32,
    pub title: String,
    pub body: String,
    pub published: bool,
}

#[derive(Insertable)]
#[table_name = "users"]
pub struct NewUser<'a> {
    pub title: &'a str,
    pub body: &'a str,
}
