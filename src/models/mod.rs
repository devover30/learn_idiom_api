use axum::async_trait;
use serde::{Deserialize, Serialize};
use sqlx::PgPool;

pub struct AppState {
    pub db: PgPool,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct IdiomReqDTO {
    pub user: String,
}

#[async_trait]
pub trait IdiomRequestMutations {
    async fn insert(&self, db: PgPool) -> Result<(), sqlx::Error>;

    async fn update(&self, db: PgPool) -> Result<(), sqlx::Error>;
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

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct IdiomRequestEntity {
    pub req_user: String,
    pub is_read: bool,
    pub idiom: IdiomDetailEntity,
}
