use axum::{
    extract::{FromRequestParts, State},
    http::{request::Parts, StatusCode},
    async_trait,
};
use std::sync::Arc;

use crate::AppState;
use ghosthub_shared::User;
use super::jwt;

#[derive(Debug, Clone)]
pub struct AuthUser(pub User);

#[async_trait]
impl FromRequestParts<Arc<AppState>> for AuthUser {
    type Rejection = StatusCode;

    async fn from_request_parts(
        parts: &mut Parts,
        state: &Arc<AppState>,
    ) -> Result<Self, Self::Rejection> {
        // Extract Bearer token from Authorization header
        let auth_header = parts
            .headers
            .get("authorization")
            .and_then(|header| header.to_str().ok())
            .ok_or(StatusCode::UNAUTHORIZED)?;

        let token = auth_header
            .strip_prefix("Bearer ")
            .ok_or(StatusCode::UNAUTHORIZED)?;

        // Verify JWT token
        let token_data = jwt::verify_jwt(token).map_err(|_| StatusCode::UNAUTHORIZED)?;

        // Load user from database
        let user = sqlx::query_as::<_, User>(
            "SELECT * FROM users WHERE id = $1 AND is_active = true"
        )
        .bind(token_data.claims.sub)
        .fetch_optional(&state.db_pool)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
        .ok_or(StatusCode::UNAUTHORIZED)?;

        Ok(AuthUser(user))
    }
}

// Optional authentication - returns None if no auth provided instead of error
#[derive(Debug, Clone)]
pub struct OptionalAuthUser(pub Option<User>);

#[async_trait]
impl FromRequestParts<Arc<AppState>> for OptionalAuthUser {
    type Rejection = StatusCode;

    async fn from_request_parts(
        parts: &mut Parts,
        state: &Arc<AppState>,
    ) -> Result<Self, Self::Rejection> {
        // Try to extract Bearer token
        let auth_header = parts
            .headers
            .get("authorization")
            .and_then(|header| header.to_str().ok());

        if let Some(header) = auth_header {
            if let Some(token) = header.strip_prefix("Bearer ") {
                // Try to verify token and load user
                if let Ok(token_data) = jwt::verify_jwt(token) {
                    if let Ok(Some(user)) = sqlx::query_as::<_, User>(
                        "SELECT * FROM users WHERE id = $1 AND is_active = true"
                    )
                    .bind(token_data.claims.sub)
                    .fetch_optional(&state.db_pool)
                    .await
                    {
                        return Ok(OptionalAuthUser(Some(user)));
                    }
                }
            }
        }

        Ok(OptionalAuthUser(None))
    }
}