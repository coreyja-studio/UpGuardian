use axum::{
    extract::State,
    response::IntoResponse,
    routing::post,
    Json, Router,
};
use cja::app_state::AppState as _;
use serde::Serialize;

use crate::{app_state::AppState, session::DBSession};

#[derive(Serialize)]
struct TestLoginResponse {
    user_id: String,
    session_id: String,
}

async fn test_login(
    cookies: tower_cookies::Cookies,
    State(app_state): State<AppState>,
) -> impl IntoResponse {
    let user_id = uuid::Uuid::new_v4();
    let coreyja_user_id = uuid::Uuid::new_v4();

    let user = sqlx::query_as::<_, (uuid::Uuid,)>(
        "INSERT INTO Users (user_id, coreyja_user_id, is_active_sponsor) VALUES ($1, $2, true) RETURNING user_id",
    )
    .bind(user_id)
    .bind(coreyja_user_id)
    .fetch_one(app_state.db())
    .await
    .unwrap();

    let session = DBSession::create(user.0, &app_state, &cookies)
        .await
        .unwrap();

    Json(TestLoginResponse {
        user_id: user.0.to_string(),
        session_id: session.session_id.to_string(),
    })
}

async fn test_reset(State(app_state): State<AppState>) -> impl IntoResponse {
    sqlx::query("DELETE FROM Checkins")
        .execute(app_state.db())
        .await
        .unwrap();
    sqlx::query("DELETE FROM Pages")
        .execute(app_state.db())
        .await
        .unwrap();
    sqlx::query("DELETE FROM Sites")
        .execute(app_state.db())
        .await
        .unwrap();
    sqlx::query("DELETE FROM Sessions")
        .execute(app_state.db())
        .await
        .unwrap();
    sqlx::query("DELETE FROM Users")
        .execute(app_state.db())
        .await
        .unwrap();

    "ok"
}

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/test/login", post(test_login))
        .route("/test/reset", post(test_reset))
}
