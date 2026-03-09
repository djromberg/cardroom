use super::endpoints;
use super::auth::create_auth_layer;

use crate::application::AuthRole;
use crate::application::ProvideServices;

use axum::Extension;
use axum::Json;
use axum::Router;
use axum::routing;
use axum_keycloak_auth::PassthroughMode;
use axum_keycloak_auth::Url;
use axum_keycloak_auth::decode::KeycloakToken;
use axum_keycloak_auth::instance::KeycloakAuthInstance;
use axum_keycloak_auth::instance::KeycloakConfig;
use axum_keycloak_auth::layer::KeycloakAuthLayer;
use log::info;
use tokio::net::TcpListener;
use tokio::sync::Mutex;

use std::io::Error;
use std::sync::Arc;


#[derive(Debug)]
pub struct AxumServer {
    port: u16,
}

impl AxumServer {
    pub fn new( port: u16) -> Self {
        Self { port }
    }

    pub async fn serve<Provider: ProvideServices + Send + 'static>(&self, provider: Provider) -> Result<(), Error> {
        let address = "0.0.0.0:".to_owned() + &self.port.to_string();
        let listener = TcpListener::bind(address).await?;

        info!("listening on {}", listener.local_addr()?);

        let router = Router::new()
            .route(
                "/tournaments",
                routing::get(endpoints::find_tournaments)
            )
            .route(
                "/tournaments",
                routing::post(endpoints::create_tournament)
            )
            .route(
                "/tournaments/{tournament_id}/join",
                routing::post(endpoints::join_tournament)
            )
            .route(
                "/tournaments/{tournament_id}/tables/{table_number}",
                routing::any(endpoints::observe_table)
            )
            .layer(create_auth_layer(vec![]))
            .with_state(Arc::new(Mutex::new(provider)));

        info!("serving cardroom application ...");

        axum::serve(listener, router).await
    }

    // pub async fn serve_protected(&self) -> Result<(), Error> {
    //     let address = "0.0.0.0:".to_owned() + &self.port.to_string();
    //     let listener = TcpListener::bind(address).await?;

    //     info!("listening on {}", listener.local_addr()?);

    //     let keycloak_auth_instance = KeycloakAuthInstance::new(
    //     KeycloakConfig::builder()
    //         .server(Url::parse("http://localhost:8080/").unwrap())
    //         .realm(String::from("cardroom"))
    //         .build(),
    //     );

    //     let router = Router::new()
    //         .route("/tournaments", routing::get(protected))
    //         .layer(create_auth_layer(keycloak_auth_instance, vec![])
    //     );

    //     info!("serving protected cardroom application ...");

    //     axum::serve(listener, router).await
    // }
}


// impl axum_keycloak_auth::role::Role for AuthRole {}


// pub async fn protected(Extension(token): Extension<KeycloakToken<AuthRole>>) -> Json<Vec<String>> {
//     Json(token.roles.iter().map(|kcr| kcr.role().to_string()).collect())
// }


// fn create_auth_layer(instance: KeycloakAuthInstance, required_roles: Vec<AuthRole>) -> KeycloakAuthLayer<AuthRole> {
//     KeycloakAuthLayer::<AuthRole>::builder()
//         .instance(instance)
//         .passthrough_mode(PassthroughMode::Block)
//         .persist_raw_claims(false)
//         .expected_audiences(vec![String::from("account")])
//         .required_roles(required_roles)
//         .build()
// }
