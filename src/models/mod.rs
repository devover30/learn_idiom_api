use std::str::FromStr;

use axum::async_trait;
use chrono::NaiveDate;
use serde::{Deserialize, Serialize};
use sqlx::PgPool;

use crate::error::AppError;

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
pub trait LoginRepo {
    async fn insert(&self, db: PgPool) -> Result<AuthSessionEntity, AppError>;
}

#[async_trait]
pub trait EntryTableMutations {
    async fn update(&self, db: PgPool) -> Result<(), sqlx::Error>;

    async fn insert(&self, db: PgPool) -> Result<(), sqlx::Error>;

    async fn delete(&self, db: PgPool) -> Result<(), sqlx::Error>;
}

#[derive(sqlx::Type, Debug, Serialize, Deserialize, Clone)]
#[sqlx(type_name = "entry_type", rename_all = "lowercase")]
pub enum EntryType {
    Income,
    Expense,
}

impl FromStr for EntryType {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "income" => Ok(EntryType::Income),
            "expense" => Ok(EntryType::Expense),
            _ => Err(()),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct EntryTypeEntity {
    pub entry_type: EntryType,
    pub category: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TokenClaims {
    pub id: String,
    pub name: String,
    pub email: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct EntryDetailEntity {
    pub id: String,
    pub ent_type: EntryTypeEntity,
    pub amount: i32,
    pub desc: String,
    pub date_add: NaiveDate,
    pub date_update: NaiveDate,
    pub user: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct EntryRequestDTO {
    pub category: String,
    pub amount: i32,
    pub date: NaiveDate,
    pub desc: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct EntryExpenseTypeSummary {
    pub category: String,
    pub total: i64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct EntryDetailSummary {
    pub total_income: i64,
    pub total_expenses: i64,
    pub balance: i64,
    pub total_entries: i64,
    pub expenses: Vec<EntryExpenseTypeSummary>,
    pub last_five: Vec<EntryDetailEntity>,
}

#[derive(Deserialize)]
pub struct EntryDetailPagination {
    pub page: Option<i32>,
}
