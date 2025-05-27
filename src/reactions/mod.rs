pub mod user;
pub mod post;

use std::collections::HashMap;
use axum::{Extension, Json};
use axum::extract::Path;
use http::StatusCode;
use serde::{Deserialize, Serialize};
use sqlx::{Pool, Postgres, Type};

#[derive(Clone, Type, Serialize, Deserialize, Eq, Hash, PartialEq)]
#[sqlx(type_name = "reaction_type", rename_all = "lowercase")]
pub enum Reaction {
    Like,
    Love,
    Sad,
    Hate,
    Funny,
    Skull,
    Cat,
}

#[derive(Serialize, Deserialize)]
pub struct ReactionCount {
    reaction: Reaction,
    count: Option<i64>,
}

#[derive(Serialize)]
pub struct PostReactions {
    id: i32,
    like: i64,
    love: i64,
    sad: i64,
    hate: i64,
    funny: i64,
    skull: i64,
    cat: i64,
}

pub async fn get_post_reactions(
    Extension(pool): Extension<Pool<Postgres>>,
    Path(id): Path<i32>,
) -> Result<Json<PostReactions>, StatusCode> {
    let reactions = sqlx::query_as!(
        ReactionCount,
        "SELECT reaction as \"reaction: _\", COUNT(*) AS count FROM reactions WHERE post_id = $1 GROUP BY reaction;",
        id
    )
        .fetch_all(&pool)
        .await
        .map_err(|_| StatusCode::NOT_FOUND)?;
    
    let mut map = reactions.into_iter().map(|r| (r.reaction, r.count)).collect::<HashMap<_, _>>();
    
    let mut get_react = |reaction: Reaction| -> i64 {
        map.remove(&reaction).unwrap_or(Some(0)).unwrap_or(0)
    };
    
    let post_reactions = PostReactions {
        id,
        like: get_react(Reaction::Like),
        love: get_react(Reaction::Love),
        sad: get_react(Reaction::Sad),
        hate: get_react(Reaction::Hate),
        funny: get_react(Reaction::Funny),
        skull: get_react(Reaction::Skull),
        cat: get_react(Reaction::Cat),
    };

    Ok(Json(post_reactions))
}