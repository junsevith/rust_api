use axum::{Extension, Json};
use axum::extract::Path;
use axum::http::StatusCode;
use serde::{Deserialize, Serialize};
use sqlx::{Pool, Postgres};
use tower::ServiceExt;
use crate::auth::CurrentUser;
use crate::posts::Post;

#[derive(Serialize, Deserialize)]
pub struct UpdatePost {
    title: String,
    body: String,
    user_id: Option<i32>,
}

pub async fn update_post(
    Extension(pool): Extension<Pool<Postgres>>,
    Extension(user): Extension<CurrentUser>,
    Path(id): Path<i32>,
    Json(updated_post): Json<UpdatePost>,
) -> Result<Json<Post>, StatusCode> {
    let check_post = sqlx::query!(
        "SELECT user_id FROM posts WHERE id = $1",
        id
    )
        .fetch_one(&pool)
        .await;
    
    let value = check_post.map_err(|_| StatusCode::NOT_FOUND)?;
    let user_id = match value.user_id {
        Some(id) => id,
        None => return Err(StatusCode::FORBIDDEN),
    };
    
    if user.id != user_id {
        return Err(StatusCode::FORBIDDEN);
    }
    
    let post = sqlx::query_as!(
        Post,
        "UPDATE posts SET title = $1, body = $2, user_id = $3 WHERE id = $4 RETURNING id, user_id, title, body",
        updated_post.title,
        updated_post.body,
        user.id,
        id
    )
        .fetch_one(&pool)
        .await;

    match post {
        Ok(post) => Ok(Json(post)),
        Err(_) => Err(StatusCode::NOT_FOUND),
    }
}

pub async fn update_post_admin(
    Extension(pool): Extension<Pool<Postgres>>,
    Path(id): Path<i32>,
    Json(updated_post): Json<UpdatePost>,
) -> Result<Json<Post>, StatusCode> {
    let post = sqlx::query_as!(
        Post,
        "UPDATE posts SET title = $1, body = $2, user_id = $3 WHERE id = $4 RETURNING id, user_id, title, body",
        updated_post.title,
        updated_post.body,
        updated_post.user_id,
        id
    )
        .fetch_one(&pool)
        .await;

    match post {
        Ok(post) => Ok(Json(post)),
        Err(_) => Err(StatusCode::NOT_FOUND),
    }
}