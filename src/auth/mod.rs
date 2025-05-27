use serde::{Deserialize, Serialize};
use sqlx::{Type};

pub(crate) mod authorization;
mod get_user;
mod jwt;
pub mod login;
pub(crate) mod admin;

#[derive(Clone, Type, Serialize, Deserialize)]
#[sqlx(type_name = "user_permission", rename_all = "lowercase")]
pub enum UserPermission {
    Admin,
    User,
    Moderator,
}

#[derive(Clone)]
pub struct CurrentUser {
    pub id: i32,
    pub email: String,
    pub username: String,
    pub password_hash: String,
    pub user_permission: UserPermission,
}
