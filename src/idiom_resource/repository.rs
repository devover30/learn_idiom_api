use sqlx::{PgPool, Row};

use crate::models::IdiomDetailEntity;

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
