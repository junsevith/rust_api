use crate::auth::CurrentUser;
use crate::reactions::Reaction;
use axum::extract::Path;
use axum::{Extension, Json};
use http::StatusCode;
use serde::{Deserialize, Serialize};
use sqlx::{Pool, Postgres};

#[derive(Serialize, Deserialize)]
pub struct React {
    pub reaction: Reaction,
}

pub async fn react(
    Extension(pool): Extension<Pool<Postgres>>,
    Path(id): Path<i32>,
    Extension(user): Extension<CurrentUser>,
    Json(react): Json<React>
) -> Result<(), StatusCode> {
    sqlx::query!(
        r#"
        INSERT INTO reactions (user_id, post_id, reaction, reacted_at)
        VALUES ($1, $2, $3, NOW())
        ON CONFLICT (user_id, post_id)
        DO UPDATE SET reaction = EXCLUDED.reaction, reacted_at = NOW()
        "#,
        user.id,
        id,
        react.reaction.clone() as Reaction,
    )
        .execute(&pool)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    
    Ok(())
}