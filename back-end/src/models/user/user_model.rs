use rocket::serde::{Serialize, Deserialize};
use rocket_sync_db_pools::diesel;
use crate::schema::users;


#[database("sqlite_db")]
struct Db(diesel::SqliteConnection);


#[derive(Debug, Clone, Deserialize, Serialize, Queryable, Insertable)]
#[serde(crate = "rocket::serde")]
#[table_name = "users"]
pub struct User {
    pub id:             i32,
    pub username:       String,
    pub first_name:     Option<String>,
    pub last_name:      Option<String>,
    pub email:          Option<String>,
    pub password:       String,
    pub is_staff:       bool,
    pub is_active:      bool,
    pub is_superuser:   bool,
    pub created_at:     String,
}


