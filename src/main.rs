use std::{env, sync::Arc};

use axum::{
    routing::{get, post},
    Router,
};
use sqlx::SqlitePool;
use theme_engine::{handler, state::AppState};
use tokio::net::TcpListener;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenvy::dotenv()?;

    let pool = SqlitePool::connect(&env::var("DATABASE_URL")?).await?;

    let state = Arc::new(AppState::new(pool));

    let app = Router::new()
        .route("/", get(handler::upcoming))
        .route("/active", get(handler::active))
        .route("/archive/culled", get(handler::culled))
        .route("/archive/previous", get(handler::previous))
        .route("/invoke_themelord", post(handler::invoke_themelord))
        .with_state(state);

    let port = env::var("PORT")?;
    let address = format!("0.0.0.0:{port}");

    let listener = TcpListener::bind(address).await?;
    axum::serve(listener, app).await?;

    Ok(())
}
