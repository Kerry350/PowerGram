use schema::users;
use serde::{Deserialize, Serialize};

// Stored user
#[derive(Debug, Queryable, Serialize)]
pub struct User {
    pub id: i32,
    pub username: String,
    pub password: String,
}

// New user
#[derive(Debug, Insertable, Deserialize)]
#[table_name="users"]
pub struct NewUser {
    pub username: String,
    pub password: String,
}

// Stored access token
#[derive(Debug, Queryable)]
pub struct AccessToken {
    pub id: i32,
    pub token: String,
    pub user_id: i32,
}

