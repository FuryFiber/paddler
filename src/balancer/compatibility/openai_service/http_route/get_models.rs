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

#[get("/v1/models")]
async fn list_models() -> Result<HttpResponse, Error> {
    let response = "hello world!";
    Ok(HttpResponse::Ok().json(response))
}
