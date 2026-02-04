use actix_web::Error;
use actix_web::HttpResponse;
use actix_web::Responder;
use actix_web::error::ErrorInternalServerError;
use actix_web::get;
use actix_web::web;
use crate::balancer::compatibility::openai_service::app_data::AppData;
use crate::agent_desired_model::AgentDesiredModel;

pub fn register(cfg: &mut web::ServiceConfig) {
    cfg.service(respond);
}

#[get("/v1/models")]
async fn respond(app_data: web::Data<AppData>) -> Result<impl Responder, Error> {
    let desired_state = app_data
        .state_database
        .read_balancer_desired_state()
        .await
        .map_err(ErrorInternalServerError)?;
    
    // Notice the & before desired_state.model
    let filename = match &desired_state.model {
        AgentDesiredModel::HuggingFace(model) => Some(model.filename.as_str()),
        AgentDesiredModel::LocalToAgent(path) => Some(path.as_str()),
        AgentDesiredModel::None => None,
    };
    
    let stem = filename
        .and_then(|f| f.split('.').next())
        .unwrap_or("none");
    
    let response = format!("{{\"data\": [{{\"id\": \"{}\", \"object\": \"model\", \"owned_by\": \"user\"}}], \"object\": \"list\"}}", stem);

    Ok(HttpResponse::Ok().json(response))
}
