use crate::users::User;
use axum::http::StatusCode;
use axum::{Extension, Json};
use bcrypt::{hash, DEFAULT_COST};
use serde::{Deserialize, Serialize};
use sqlx::error::ErrorKind;
use sqlx::{Error, Pool, Postgres};

#[derive(Serialize, Deserialize)]
pub struct CreateUser {
    username: String,
    email: String,
    password: String,   
}

pub async fn create_user(
    Extension(pool): Extension<Pool<Postgres>>,
    Json(new_user): Json<CreateUser>,
) -> Result<Json<User>, StatusCode> {
    
    let hashed_password = hash(new_user.password, DEFAULT_COST)
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    
    let user = sqlx::query_as!(
        User,
        "INSERT INTO users (username, email, password_hash) VALUES ($1, $2, $3) RETURNING id, username, email, user_permission as \"user_permission: _\"",
        new_user.username,
        new_user.email,
        hashed_password
    )
    .fetch_one(&pool)
    .await
    .map_err(|err| match err {
        Error::Database(err) => match err.kind(){
            ErrorKind::UniqueViolation => StatusCode::CONFLICT,
            _ => StatusCode::INTERNAL_SERVER_ERROR,
        },
        _ => StatusCode::INTERNAL_SERVER_ERROR,
    })?;

    Ok(Json(user))
}
