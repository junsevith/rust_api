use crate::auth::CurrentUser;
use crate::reactions::Reaction;
use axum::{Extension, Json};
use chrono::NaiveDateTime;
use http::StatusCode;
use serde::{Deserialize, Serialize};
use sqlx::{Pool, Postgres};

#[derive(Serialize, Deserialize)]
pub struct UserReaction {
    post_id: i32,
    reaction: Reaction,
    reacted_at: Option<NaiveDateTime>
}

pub async fn get_user_reactions(
    Extension(pool): Extension<Pool<Postgres>>,
    Extension(user): Extension<CurrentUser>,
) -> Result<Json<Vec<UserReaction>>, StatusCode> {
    let reactions = sqlx::query_as!(
        UserReaction,
        r#"
        SELECT post_id, reaction as "reaction: _", reacted_at
        FROM reactions
        WHERE user_id = $1
        ORDER BY reacted_at DESC
        "#,
        user.id
    )
        .fetch_all(&pool)
        .await
        .map_err(|_| StatusCode::NOT_FOUND)?;

    Ok(Json(reactions))
}