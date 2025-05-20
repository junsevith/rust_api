use crate::auth::CurrentUser;
use crate::users::User;
use axum::{Extension, Json};

pub async fn get_user(
    Extension(user): Extension<CurrentUser>,
) -> Json<User> {
    
    Json(User {
        id: user.id,
        username: user.username,
        email: user.email,
        user_permission: user.user_permission,
    })
    
}