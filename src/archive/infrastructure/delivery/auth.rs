use crate::application::AuthRole;

use axum_keycloak_auth::PassthroughMode;
use axum_keycloak_auth::Url;
use axum_keycloak_auth::instance::KeycloakAuthInstance;
use axum_keycloak_auth::instance::KeycloakConfig;
use axum_keycloak_auth::layer::KeycloakAuthLayer;


pub fn create_auth_layer(required_roles: Vec<AuthRole>) -> KeycloakAuthLayer<AuthRole> {
    let instance = KeycloakAuthInstance::new(
        KeycloakConfig::builder()
            .server(Url::parse("https://keycloak.riensberg.lan/").unwrap())
            .realm(String::from("cardroom"))
            .build(),
        );
    KeycloakAuthLayer::<AuthRole>::builder()
        .instance(instance)
        .passthrough_mode(PassthroughMode::Block)
        .persist_raw_claims(false)
        .expected_audiences(vec![String::from("account")])
        .required_roles(required_roles)
        .build()
}


impl axum_keycloak_auth::role::Role for AuthRole {}
