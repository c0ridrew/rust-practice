use crate::schema::users;
use serde_derive::Deserialize;

#[derive(Queryable, Deserialize)]
pub struct User {
    pub id: i32,
    pub email: String,
}

#[derive(Insertable, Deserialize)]
#[table_name = "users"]
pub struct NewUser {
    pub email: String,
}
