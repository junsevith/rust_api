use axum::{Extension, Json};
use axum::http::StatusCode;
use serde::{Deserialize, Serialize};
use sqlx::{Pool, Postgres};
use crate::auth::CurrentUser;
use crate::posts::Post;

#[derive(Serialize, Deserialize)]
pub struct CreatePost {
    title: String,
    body: String,
}

pub async fn create_post(
    Extension(pool): Extension<Pool<Postgres>>,
    Extension(user): Extension<CurrentUser>,
    Json(new_post): Json<CreatePost>,
) -> Result<Json<Post>, StatusCode> {
    let post = sqlx::query_as!(
        Post,
        "INSERT INTO posts (user_id, title, body) VALUES ($1, $2, $3) RETURNING id, title, body, user_id",
        user.id,
        new_post.title,
        new_post.body
    )
        .fetch_one(&pool)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(post))
}