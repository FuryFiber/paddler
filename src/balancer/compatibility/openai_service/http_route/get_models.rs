use std::time::SystemTime;
use std::time::UNIX_EPOCH;

use actix_web::Error;
use actix_web::HttpResponse;
use actix_web::get;
use actix_web::web;
use anyhow::anyhow;
use async_trait::async_trait;
use nanoid::nanoid;
use serde::Deserialize;
use serde_json::json;
use tokio_stream::StreamExt as _;

use crate::balancer::management_service::app_data::AppData;

pub fn register(cfg: &mut web::ServiceConfig) {
    cfg.service(list_models);
}

fn current_timestamp() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards")
        .as_secs()
}

#[get("/v1/models")]
async fn list_models(
    app_data: web::Data<AppData>,
) -> Result<HttpResponse, Error> {
    let desired_state = app_data
        .state_database
        .read_balancer_desired_state()
        .await
        .map_err(actix_web::error::ErrorInternalServerError)?;

    Ok(HttpResponse::Ok().json("hello world!"))

}
