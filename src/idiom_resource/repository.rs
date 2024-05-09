use axum::async_trait;
use sqlx::{PgPool, Row};

use crate::models::{IdiomDetailEntity, IdiomRequestEntity, IdiomRequestMutations};

pub async fn select_all_idioms(db: PgPool) -> Result<Vec<IdiomDetailEntity>, sqlx::Error> {
    let rows = sqlx::query(
        r"
        SELECT id,idiom_eng,idiom_hin FROM idioms_tbl
        ",
    )
    .fetch_all(&db)
    .await?;

    let mut idiom_list: Vec<IdiomDetailEntity> = Vec::new();

    for row in rows {
        let entity = IdiomDetailEntity {
            id: row.get("id"),
            idiom_eng: row.get("idiom_eng"),
            idiom_hin: row.get("idiom_hin"),
        };
        idiom_list.push(entity);
    }

    Ok(idiom_list)
}

pub async fn select_all_idioms_not_read(
    db: PgPool,
) -> Result<Vec<IdiomRequestEntity>, sqlx::Error> {
    let query = r"
        SELECT ir.req_user,ir.is_read,i.id,i.idiom_eng,i.idiom_hin
        FROM req_tbl AS ir LEFT JOIN idioms_tbl AS i
        ON ir.idiom = i.id WHERE ir.is_read = $1
        ";

    let rows = sqlx::query(query).bind(false).fetch_all(&db).await?;

    let mut idiom_req_list: Vec<IdiomRequestEntity> = Vec::new();

    for row in rows {
        let entity = IdiomRequestEntity {
            req_user: row.get("req_user"),
            is_read: row.get("is_read"),
            idiom: IdiomDetailEntity {
                id: row.get("id"),
                idiom_eng: row.get("idiom_eng"),
                idiom_hin: row.get("idiom_hin"),
            },
        };
        idiom_req_list.push(entity);
    }

    Ok(idiom_req_list)
}

#[async_trait]
impl IdiomRequestMutations for IdiomRequestEntity {
    async fn insert(&self, db: PgPool) -> Result<(), sqlx::Error> {
        sqlx::query(
            r"
            INSERT INTO req_tbl
            (req_user,is_read,idiom)
            VALUES
            ($1,$2,$3)
            ",
        )
        .bind(self.req_user.clone())
        .bind(self.is_read)
        .bind(self.idiom.id.clone())
        .execute(&db.clone())
        .await?;

        Ok(())
    }

    async fn update(&self, db: PgPool) -> Result<(), sqlx::Error> {
        sqlx::query(
            r"
            UPDATE req_tbl SET
            is_read = $1 WHERE req_user = $2 AND idiom = $3
            ",
        )
        .bind(self.is_read)
        .bind(self.req_user.clone())
        .bind(self.idiom.id.clone())
        .execute(&db)
        .await?;

        Ok(())
    }
}
