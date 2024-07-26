use std::{env, sync::Arc};

use axum::{extract::State, Json};
use rand::{distributions::Slice, Rng};
use reqwest::StatusCode;
use serde_json::{json, Value};

use crate::{state::AppState, theme::Theme};

pub async fn active(State(state): State<Arc<AppState>>) -> Result<String, StatusCode> {
    sqlx::query_scalar!(
        r#"
    SELECT title
    FROM theme
    WHERE activated IS NOT NULL AND NOT(culled)
    ORDER BY activated DESC
    LIMIT 1"#
    )
    .fetch_one(&state.pool)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)
}

pub async fn culled(State(state): State<Arc<AppState>>) -> Result<Json<Value>, StatusCode> {
    sqlx::query_as!(Theme, "SELECT * FROM theme WHERE culled")
        .fetch_all(&state.pool)
        .await
        .map(|rows| Json(json!(rows)))
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)
}

pub async fn invoke_themelord(State(state): State<Arc<AppState>>) -> Result<String, StatusCode> {
    let upcoming_themes =
        sqlx::query_scalar!("SELECT title FROM theme WHERE activated IS NULL AND NOT(culled)")
            .fetch_all(&state.pool)
            .await
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let mut rng = state.rng.lock().await;

    let new_theme_title = (&mut *rng)
        .sample_iter(Slice::new(&upcoming_themes).map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?)
        .next()
        .unwrap()
        .to_string();

    sqlx::query!(
        "UPDATE theme SET activated = current_date WHERE title = $1",
        new_theme_title
    )
    .execute(&state.pool)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let content = format!(
        r#"@here
        
        :bell: :bell: :bell: HEAR YE HEAR YE :bell: :bell: :bell:
        
        Your new theme is...
        
        **{new_theme_title}**!
        
        Yours truly,
        
        Themelord69
        
        Master Rethemer, Grade II
        
        Autocratic People's Republic of Rethemers"#
    );

    state
        .client
        .post(env::var("THEMELORD_URL").map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?)
        .json(&json!({
            "content": content
        }))
        .send()
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(new_theme_title)
}

pub async fn previous(State(state): State<Arc<AppState>>) -> Result<Json<Value>, StatusCode> {
    sqlx::query_as!(
        Theme,
        "SELECT * FROM theme WHERE activated IS NOT NULL ORDER BY activated DESC"
    )
    .fetch_all(&state.pool)
    .await
    .map(|rows| Json(json!(rows)))
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)
}

pub async fn upcoming(State(state): State<Arc<AppState>>) -> Result<Json<Value>, StatusCode> {
    sqlx::query_as!(
        Theme,
        "SELECT * FROM theme WHERE activated IS NULL AND NOT(culled)"
    )
    .fetch_all(&state.pool)
    .await
    .map(|rows| Json(json!(rows)))
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)
}
