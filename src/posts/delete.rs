use axum::extract::Path;
use axum::http::StatusCode;
use axum::{Extension, Json};
use sqlx::{Pool, Postgres};
use crate::auth::CurrentUser;

pub async fn delete_post(
    Extension(pool): Extension<Pool<Postgres>>,
    Extension(user): Extension<CurrentUser>,
    Path(id): Path<i32>,
) -> Result<Json<serde_json::Value>, StatusCode> {
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
    
    let result = sqlx::query!("DELETE FROM posts WHERE id = $1", id)
        .execute(&pool)
        .await;

    match result {
        Ok(_) => Ok(Json(serde_json::json! ({
            "message": "Post deleted successfully"
        }))),
        Err(_) => Err(StatusCode::NOT_FOUND),
    }
}

pub async fn delete_post_admin(
    Extension(pool): Extension<Pool<Postgres>>,
    Path(id): Path<i32>,
) -> Result<Json<serde_json::Value>, StatusCode> {
    let result = sqlx::query!("DELETE FROM posts WHERE id = $1", id)
        .execute(&pool)
        .await;

    match result {
        Ok(_) => Ok(Json(serde_json::json! ({
            "message": "Post deleted successfully"
        }))),
        Err(_) => Err(StatusCode::NOT_FOUND),
    }
}