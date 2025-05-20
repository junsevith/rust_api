use axum::http::StatusCode;
use axum::{Extension, Json};
use bcrypt::verify;
use serde::Deserialize;
use sqlx::{Pool, Postgres};
use crate::auth::get_user::retrieve_user_by_email;
use crate::auth::jwt::encode_jwt;

#[derive(Deserialize)]
pub struct SignInData {
    pub email: String,
    pub password: String,
}

pub async fn login(
    Extension(pool): Extension<Pool<Postgres>>,
    Json(user_data): Json<SignInData>,
) -> Result<Json<String>, StatusCode> {
    // 1. Retrieve user from the database
    let user = match retrieve_user_by_email(&user_data.email, pool).await {
        Some(user) => user,
        None => return Err(StatusCode::UNAUTHORIZED), // User not found
    };

    // 2. Compare the password
    if !verify(&user_data.password, &user.password_hash)
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
    // Handle bcrypt errors
    {
        return Err(StatusCode::UNAUTHORIZED); // Wrong password
    }

    // 3. Generate JWT
    let token = encode_jwt(user.email).map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    // 4. Return the token
    Ok(Json(token))
}
