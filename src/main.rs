mod posts;
mod users;
mod auth;
mod reactions;

use axum::{routing::get, Extension, Router};
use axum::routing::{post, put};
use dotenvy::dotenv;
use sqlx::postgres::PgPoolOptions;
use tower::ServiceBuilder;
use tracing::{info, Level};
use crate::auth::login::login;
use crate::posts::{get_post};
use crate::posts::create::create_post;
use crate::posts::delete::{delete_post, delete_post_admin};
use crate::posts::many::get_posts;
use crate::posts::update::{update_post, update_post_admin};
use crate::reactions::get_post_reactions;
use crate::reactions::post::react;
use crate::reactions::user::get_user_reactions;
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
        .route("/posts/{id}/reactions", get(get_post_reactions))
        .route("/user", post(create_user))
        .route("/login", post(login))
        // Extension Layer
        .layer(Extension(pool.clone()));

    let user= Router::new()
        .route("/posts", post(create_post))
        .route("/posts/{id}", put(update_post).delete(delete_post))
        .route("/posts/{id}/reactions", post(react))
        .route("/user", get(get_user))
        .route("/user/reactions", get(get_user_reactions))
        // Extension Layer
        .layer(ServiceBuilder::new()
            .layer(Extension(pool.clone()))
            .layer(axum::middleware::from_fn_with_state(pool.clone(), auth::authorization::authorization_middleware))
        );
    
    let admin = Router::new()
        .route("/posts/{id}", put(update_post_admin).delete(delete_post_admin))
        // Extension Layer
        .layer(ServiceBuilder::new()
            .layer(Extension(pool.clone()))
            .layer(axum::middleware::from_fn_with_state(pool.clone(), auth::authorization::authorization_middleware))
            .layer(axum::middleware::from_fn_with_state(pool.clone(), auth::admin::auth_admin))
        );

    let app = guest.merge(user).merge(admin);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:5000").await?;
    info!("Server is running on http://0.0.0.0:5000");
    axum::serve(listener, app).await?;

    Ok(())
}