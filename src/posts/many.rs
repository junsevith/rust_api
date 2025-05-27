use crate::posts::Post;
use axum::{Extension, Json};
use http::StatusCode;
use sqlx::{Pool, Postgres};

use serde::Deserialize;
use sqlx_conditional_queries::conditional_query_as;
use tracing::trace;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SortBy {
    Id,
    Title,
    UserId,
    CreatedAt,
}

impl SortBy {
    fn as_str(&self) -> &'static str {
        match self {
            SortBy::Id => "id",
            SortBy::Title => "title",
            SortBy::UserId => "user_id",
            SortBy::CreatedAt => "created_at",
        }
    }
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum SortOrder {
    Asc,
    Desc,
}

impl SortOrder {
    fn as_str(&self) -> &'static str {
        match self {
            SortOrder::Asc => "ASC",
            SortOrder::Desc => "DESC",
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct PostQuery {
    page: Option<u32>,
    limit: Option<u32>,
    sort_by: Option<SortBy>,
    order: Option<SortOrder>,
    user_id: Option<i32>,
}

pub async fn get_posts(
    Extension(pool): Extension<Pool<Postgres>>,
    Json(params): Json<PostQuery>,
) -> Result<Json<Vec<Post>>, StatusCode> {
    let page = params.page.unwrap_or(1).max(1) as i64;
    let limit = params.limit.unwrap_or(10).min(100) as i64;
    let offset = (page - 1) * limit ;

    let sort_by = params.sort_by.unwrap_or(SortBy::Id);
    let order = params.order.unwrap_or(SortOrder::Asc);
    
    trace!("{}",sort_by.as_str());
    
    let posts = conditional_query_as!(
        Post,
        r#"SELECT id, user_id, title, body 
        FROM posts
         {#filter}
         ORDER BY {#sort_by} {#order}
         LIMIT {limit} OFFSET {offset}"#,
        #filter = match params.user_id {
            Some(user_id) => "WHERE user_id = {user_id}",
            None => "",
        },
        #sort_by = match sort_by {
            SortBy::Id => "id",
            SortBy::Title => "title",
            SortBy::UserId => "user_id",
            SortBy::CreatedAt => "created_at",
        },
        #order = match order {
            SortOrder::Asc => "ASC",
            SortOrder::Desc => "DESC",
        },
    ).fetch_all(&pool).await.map_err(|_| StatusCode::NOT_FOUND)?;
    
    
    Ok(Json(posts))
    
}
