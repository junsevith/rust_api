pub mod create;
pub mod get_user;

use crate::auth::UserPermission;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct User {
    id: i32,
    username: String,
    email: String,
    pub user_permission: UserPermission,
}
