use axum::async_trait;
use serde::{Deserialize, Serialize};
use sqlx::PgPool;

pub struct AppState {
    pub db: PgPool,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AuthSessionDTO {
    pub session_id: String,
    pub code_challenge: String,
    pub code_verifier: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AuthSessionEntity {
    pub id: i32,
    pub auth_session: AuthSessionDTO,
}

#[async_trait]
pub trait EntryTableMutations {
    async fn update(&self, db: PgPool) -> Result<(), sqlx::Error>;

    async fn insert(&self, db: PgPool) -> Result<(), sqlx::Error>;

    async fn delete(&self, db: PgPool) -> Result<(), sqlx::Error>;
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TokenClaims {
    pub id: String,
    pub name: String,
    pub email: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct IdiomDetailEntity {
    pub id: String,
    pub idiom_eng: String,
    pub idiom_hin: String,
}
