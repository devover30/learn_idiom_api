use std::sync::Arc;

use axum::{extract::State, Form, Json};

use crate::{
    error::AppError,
    models::{AppState, IdiomDetailEntity, IdiomReqDTO},
};

pub async fn get_idiom_by_user(
    State(data): State<Arc<AppState>>,
    Form(req_dto): Form<IdiomReqDTO>,
) -> Result<Json<IdiomDetailEntity>, AppError> {
    todo!()
}
//let mut rng = rand::thread_rng();
//let random_string_index: usize = rng.gen_range(0..idioms_list.len());
//println!("{:?}", idioms_list[random_string_index]);
