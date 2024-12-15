use std::sync::Arc;

use async_trait::async_trait;
use axum::{
    extract::FromRequestParts,
    http::{request::Parts, StatusCode},
};
use shield::Shield;

pub struct ExtractShield(pub Arc<Shield>);

#[async_trait]
impl<S: Send + Sync> FromRequestParts<S> for ExtractShield {
    type Rejection = (StatusCode, &'static str);

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        parts
            .extensions
            .get::<Arc<Shield>>()
            .cloned()
            .map(ExtractShield)
            .ok_or((
                StatusCode::INTERNAL_SERVER_ERROR,
                "Can't extract Shield. Is `ShieldLayer` enabled?",
            ))
    }
}