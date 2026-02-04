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

    // For simplicity, we assume that the model is always ready to use.
    let model_name = match &desired_state.model {
        crate::agent_desired_model::AgentDesiredModel::HuggingFace(hf_model_ref) => {
            format!("hf_{}", hf_model_ref.repo_id.replace('/', "_"))
        }
        crate::agent_desired_model::AgentDesiredModel::LocalToAgent(local_model_name) => {
            local_model_name.clone()
        }
        crate::agent_desired_model::AgentDesiredModel::None => {
            return Err(actix_web::error::ErrorInternalServerError(
                "No model specified in desired state",
            ));
        }
    };

    let response = json!({
        "data": [
            {
                "id": model_name,
                "object": "model",
                "created": current_timestamp(),
                "owned_by": "user",
                "permission": [],
            }
        ],
        "object": "list",
    });

    Ok(HttpResponse::Ok().json(response))

}
