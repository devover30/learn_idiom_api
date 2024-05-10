use crate::{
    error::AppError,
    idiom_resource::repository::select_all_idioms,
    models::{AppState, IdiomDetailEntity, IdiomReqDTO, IdiomRequestEntity, IdiomRequestMutations},
};
use axum::{extract::State, Form, Json};
use rand::Rng;
use std::sync::Arc;

pub async fn get_idiom_by_user(
    State(data): State<Arc<AppState>>,
    Form(req_dto): Form<IdiomReqDTO>,
) -> Result<Json<IdiomDetailEntity>, AppError> {
    let mut idiom_list = match select_all_idioms(data.db.clone()).await {
        Ok(list) => list,
        Err(err) => {
            tracing::error!("{:?}", err);
            return Err(AppError::DatabaseError);
        }
    };

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
