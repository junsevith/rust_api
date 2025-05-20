mod posts;
mod users;
mod auth;

use axum::{routing::get, Extension, Router};
use axum::routing::{post, put};
use dotenvy::dotenv;
use sqlx::postgres::PgPoolOptions;
use tower::ServiceBuilder;
use tracing::{info, Level};
use crate::auth::login::login;
use crate::posts::{get_post, get_posts};
use crate::posts::create::create_post;
use crate::posts::delete::delete_post;
use crate::posts::update::update_post;
use crate::users::create::create_user;
use crate::users::get_user::get_user;

#[tokio::main]
async fn main() -> Result<(), sqlx::Error> {
    dotenv().ok();
    let url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let pool = PgPoolOptions::new().connect(&url).await?;

    tracing_subscriber::fmt().with_max_level(Level::TRACE).init();

    let guest = Router::new()
        .route("/posts", get(get_posts))
        .route("/posts/{id}", get(get_post))
        .route("/user", post(create_user))
        .route("/login", post(login))
        // Extension Layer
        .layer(Extension(pool.clone()));

    let user= Router::new()
        .route("/posts", post(create_post))
        .route("/posts/{id}", put(update_post).delete(delete_post))
        .route("/user", get(get_user))
        // Extension Layer
        .layer(ServiceBuilder::new()
            .layer(Extension(pool.clone()))
            .layer(axum::middleware::from_fn_with_state(pool.clone(), auth::authorization::authorization_middleware))
        );

    let app = guest.merge(user);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:5000").await?;
    info!("Server is running on http://0.0.0.0:5000");
    axum::serve(listener, app).await?;

    Ok(())
}