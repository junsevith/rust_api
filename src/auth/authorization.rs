use crate::auth::get_user::retrieve_user_by_email;
use crate::auth::jwt::decode_jwt;
use axum::body::Body;
use axum::extract::Request;
use axum::middleware::Next;
use axum::response::{IntoResponse, Response};
use axum::{Extension, Json};
use http::StatusCode;
use serde_json::json;
use sqlx::{Pool, Postgres};

pub struct AuthError {
    message: String,
    status_code: StatusCode,
}

impl IntoResponse for AuthError {
    fn into_response(self) -> Response<Body> {
        let body = Json(json!({
            "error": self.message,
        }));

        (self.status_code, body).into_response()
    }
}

pub async fn authorization_middleware(
    Extension(pool): Extension<Pool<Postgres>>,
    mut req: Request,
    next: Next,
) -> Result<Response<Body>, AuthError> {
    let auth_header = req.headers_mut().get(http::header::AUTHORIZATION);
    let auth_header = match auth_header {
        Some(header) => header.to_str().map_err(|_| AuthError {
            message: "Empty header is not allowed".to_string(),
            status_code: StatusCode::FORBIDDEN,
        })?,
        None => {
            return Err(AuthError {
                message: "Please add the JWT token to the header".to_string(),
                status_code: StatusCode::FORBIDDEN,
            });
        }
    };
    let mut header = auth_header.split_whitespace();
    let (_bearer, token) = (header.next(), header.next());
    let token_data = match decode_jwt(token.unwrap().to_string()) {
        Ok(data) => data,
        Err(_) => {
            return Err(AuthError {
                message: "Unable to decode token".to_string(),
                status_code: StatusCode::UNAUTHORIZED,
            });
        }
    };
    // Fetch the user details from the database
    let current_user = match retrieve_user_by_email(&token_data.claims.email, pool).await {
        Some(user) => user,
        None => {
            return Err(AuthError {
                message: "You are not an authorized user".to_string(),
                status_code: StatusCode::UNAUTHORIZED,
            });
        }
    };
    req.extensions_mut().insert(current_user);
    Ok(next.run(req).await)
}
