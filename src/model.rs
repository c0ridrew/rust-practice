use serde::{Deserialize, Serialize};

use crate::schema::users;

#[derive(Queryable, Deserialize, Serialize)]
pub struct User {
    pub id: i32,
    pub email: String,
}

#[derive(Insertable, Deserialize, Serialize)]
#[table_name = "users"]
pub struct NewUser {
    pub email: String,
}
