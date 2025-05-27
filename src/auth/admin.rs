use crate::auth::authorization::AuthError;
use crate::auth::UserPermission::Admin;
use crate::auth::CurrentUser;
use axum::body::Body;
use axum::extract::Request;
use axum::middleware::Next;
use axum::response::Response;
use axum::Extension;
use http::StatusCode;

pub async fn auth_admin(
    mut req: Request,
    next: Next,
) -> Result<Response<Body>, AuthError> {
    let user_type = req.extensions().get::<Extension<CurrentUser>>().unwrap().user_permission.clone();
    match user_type {
        Admin => {
            //OK
        }
        _ => {
            return Err(AuthError {
                message: "You are not authorized to access this resource".to_string(),
                status_code: StatusCode::FORBIDDEN,
            });
        }
    }
    Ok(next.run(req).await)
}