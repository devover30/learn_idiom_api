use crate::{
    error::AppError,
    idiom_resource::repository::select_all_idioms,
    models::{AppState, IdiomDetailEntity, IdiomReqDTO, IdiomRequestEntity, IdiomRequestMutations},
};
use axum::{
    extract::{Path, State},
    Form, Json,
};
use rand::Rng;
use std::sync::Arc;

use super::repository::{select_all_idioms_not_read, select_idiom_req_by_id};

pub async fn get_idiom_by_user(
    State(data): State<Arc<AppState>>,
    Form(req_dto): Form<IdiomReqDTO>,
) -> Result<Json<IdiomDetailEntity>, AppError> {
    let mut idiom_req_list =
        match select_all_idioms_not_read(data.db.clone(), req_dto.user.clone()).await {
            Ok(list) => list,
            Err(err) => {
                tracing::error!("{:?}", err);
                return Err(AppError::DatabaseError);
            }
        };

    if idiom_req_list.len() > 0 {
        let not_read_idiom = idiom_req_list.remove(0).idiom;
        return Ok(Json(not_read_idiom));
    }

    let mut idiom_list = match select_all_idioms(data.db.clone()).await {
        Ok(list) => list,
        Err(err) => {
            tracing::error!("{:?}", err);
            return Err(AppError::DatabaseError);
        }
    };

    /*
     * As rand doesn't impl 'Send' Trait i had to encapsulate non-async
     * code in another function and then call that function
     * from here (async function)
     */

    let idiom = idiom_list.remove(get_random_string(idiom_list.len()));

    let idiom_req = IdiomRequestEntity {
        req_user: req_dto.user.clone(),
        is_read: false,
        idiom: idiom.clone(),
    };

    match idiom_req.insert(data.db.clone()).await {
        Ok(_) => return Ok(Json(idiom)),
        Err(err) => {
            tracing::error!("{:?}", err);
            return Err(AppError::DatabaseError);
        }
    };
}

fn get_random_string(len: usize) -> usize {
    let mut rng = rand::thread_rng();
    let random_string_index: usize = rng.gen_range(0..len);
    random_string_index
}

pub async fn update_idiom_read_action(
    State(data): State<Arc<AppState>>,
    Path(idiom_id): Path<String>,
    Form(req_dto): Form<IdiomReqDTO>,
) -> Result<(), AppError> {
    let mut idiom_req = match select_idiom_req_by_id(data.db.clone(), idiom_id, req_dto.user).await
    {
        Ok(entity) => entity,
        Err(err) => {
            tracing::error!("{:?}", err);
            if let sqlx::Error::RowNotFound = err {
                return Err(AppError::NotFoundError);
            }
            return Err(AppError::DatabaseError);
        }
    };

    idiom_req.is_read = true;

    match idiom_req.update(data.db.clone()).await {
        Ok(_) => (),
        Err(err) => {
            tracing::error!("{:?}", err);
            return Err(AppError::DatabaseError);
        }
    }

    Ok(())
}
