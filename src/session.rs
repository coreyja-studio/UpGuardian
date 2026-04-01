use std::convert::Infallible;

use axum::{
    extract::{FromRequestParts, OptionalFromRequestParts},
    http::request::Parts,
    response::{IntoResponse, Redirect, Response},
};
use cja::app_state::AppState as _;
use uuid::Uuid;

use crate::app_state::AppState;

/// Database-backed session with user association.
/// Replaces the old cja::server::session::DBSession that no longer exists.
#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct DBSession {
    pub session_id: Uuid,
    pub user_id: Uuid,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

/// Rejection type that redirects to login page.
pub struct SessionRedirect(Redirect);

impl IntoResponse for SessionRedirect {
    fn into_response(self) -> Response {
        self.0.into_response()
    }
}

impl From<sqlx::Error> for SessionRedirect {
    fn from(_: sqlx::Error) -> Self {
        SessionRedirect(Redirect::temporary("/login"))
    }
}

impl DBSession {
    pub async fn create(
        user_id: Uuid,
        app_state: &AppState,
        cookies: &tower_cookies::Cookies,
    ) -> color_eyre::Result<Self> {
        let session_id = Uuid::new_v4();
        let row = sqlx::query_as::<_, (Uuid, Uuid, chrono::DateTime<chrono::Utc>, chrono::DateTime<chrono::Utc>)>(
            "INSERT INTO Sessions (session_id, user_id, created_at, updated_at) VALUES ($1, $2, NOW(), NOW()) RETURNING session_id, user_id, created_at, updated_at"
        )
        .bind(session_id)
        .bind(user_id)
        .fetch_one(app_state.db())
        .await?;

        let session = DBSession {
            session_id: row.0,
            user_id: row.1,
            created_at: row.2,
            updated_at: row.3,
        };

        let cookie = tower_cookies::Cookie::build(("session_id", session.session_id.to_string()))
            .path("/")
            .http_only(true)
            .secure(true);

        let private = cookies.private(&app_state.cookie_key().0);
        private.add(cookie.into());

        Ok(session)
    }

    async fn from_parts(parts: &mut Parts, state: &AppState) -> Result<Option<Self>, Infallible> {
        let cookies = match tower_cookies::Cookies::from_request_parts(parts, state).await {
            Ok(c) => c,
            Err(_) => return Ok(None),
        };

        let private = cookies.private(&state.cookie_key().0);

        let session_cookie = match private.get("session_id") {
            Some(c) => c,
            None => return Ok(None),
        };

        let session_id = match Uuid::parse_str(session_cookie.value()) {
            Ok(id) => id,
            Err(_) => return Ok(None),
        };

        let row = sqlx::query_as::<_, (Uuid, Uuid, chrono::DateTime<chrono::Utc>, chrono::DateTime<chrono::Utc>)>(
            "SELECT session_id, user_id, created_at, updated_at FROM Sessions WHERE session_id = $1"
        )
        .bind(session_id)
        .fetch_optional(state.db())
        .await;

        match row {
            Ok(Some(row)) => Ok(Some(DBSession {
                session_id: row.0,
                user_id: row.1,
                created_at: row.2,
                updated_at: row.3,
            })),
            _ => Ok(None),
        }
    }
}

impl OptionalFromRequestParts<AppState> for DBSession {
    type Rejection = Infallible;

    async fn from_request_parts(
        parts: &mut Parts,
        state: &AppState,
    ) -> Result<Option<Self>, Self::Rejection> {
        Self::from_parts(parts, state).await
    }
}

impl FromRequestParts<AppState> for DBSession {
    type Rejection = SessionRedirect;

    async fn from_request_parts(
        parts: &mut Parts,
        state: &AppState,
    ) -> Result<Self, Self::Rejection> {
        match Self::from_parts(parts, state).await {
            Ok(Some(session)) => Ok(session),
            _ => Err(SessionRedirect(Redirect::temporary("/login"))),
        }
    }
}
