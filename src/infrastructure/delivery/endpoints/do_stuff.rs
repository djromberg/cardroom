use std::time::Duration;

use super::create_auth_info;

use crate::application::ApplicationError;
use crate::application::AuthRole;
use crate::application::DoStuff;

use axum::extract;
use axum_keycloak_auth::decode::KeycloakToken;
use serde::Deserialize;
use serde::Serialize;
use tokio::time::sleep;
use uuid::Uuid;


#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DoStuffRequest {
    pub do_it_cool: bool,
}


#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DoStuffResponse {
    pub result_uuid: Uuid,
}


pub async fn do_stuff<Service: DoStuff>(
    extract::State(service): extract::State<Service>,
    extract::Extension(token): extract::Extension<KeycloakToken<AuthRole>>,
    extract::Json(request): extract::Json<DoStuffRequest>,
) -> Result<extract::Json<DoStuffResponse>, ApplicationError> {
    let auth_info = create_auth_info(token)?;
    let uuid = service.do_stuff(request.do_it_cool, &auth_info).await?;
    sleep(Duration::from_millis(250)).await;
    let response = DoStuffResponse { result_uuid: uuid };
    Ok(extract::Json(response))
}
