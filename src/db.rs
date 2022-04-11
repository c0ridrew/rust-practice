use diesel::mysql::MysqlConnection;
use diesel::prelude::*;
use dotenv::dotenv;

pub fn establish_connection() -> MysqlConnection {
    dotenv().ok();
    let database_url = "mysql://root:password@127.0.0.1:3306/rust_practice";

    MysqlConnection::establish(&database_url)
        .expect(&format!("Error connecting to {}", database_url))
}
