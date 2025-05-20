use sqlx::{Pool, Postgres};
use tracing::{error, trace};
use crate::auth::CurrentUser;

pub async fn retrieve_user_by_email(
    email: &str,
    pool: Pool<Postgres>,
) -> Option<CurrentUser> {
    let user = sqlx::query_as!(
        CurrentUser,
        "SELECT id, email, username, password_hash, user_permission as \"user_permission: _\" FROM users WHERE email = $1",
        email
    )
        .fetch_one(&pool)
        .await;
    
    match user {
        Ok(user) => Some(user),
        Err(val) => {
            error!("User not found {}", val);
            None
        }
    }
}