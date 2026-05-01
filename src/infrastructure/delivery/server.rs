use axum::extract;
use axum::Router;
use axum::routing;
use log::info;
use tokio::net::TcpListener;
use uuid::Uuid;

use std::io::Error;

use crate::application::ActOnTable;
use crate::application::AuthInfo;
use crate::application::AuthRole;
use crate::application::CreateTournament;
use crate::application::CreateTournamentRequest;
use crate::application::CreateTournamentResponse;
use crate::application::ProvideTableServices;
use crate::application::ProvideTournamentServices;
use crate::domain::TableId;


#[derive(Debug)]
pub struct AxumServer<TournamentServices, TableServices> {
    tournament_services: TournamentServices,
    table_services: TableServices,
    port: u16,
}


impl<TournamentServices: ProvideTournamentServices, TableServices: ProvideTableServices> AxumServer<TournamentServices, TableServices> {
    pub fn new(tournament_services: TournamentServices, table_services: TableServices, port: u16) -> Self {
        Self { tournament_services, table_services, port }
    }

    pub async fn serve(&self) -> Result<(), Error> {
        let address = "0.0.0.0:".to_owned() + &self.port.to_string();
        let listener = TcpListener::bind(address).await?;

        info!("listening on {}", listener.local_addr()?);

        let router = Router::new()
            .route(
                "/tournaments",
                routing::post(create_tournament::<TournamentServices::CreateTournamentServiceType>)
            )
            .with_state(self.tournament_services.create_tournament_service())
            .route(
                "/tables/{table_id}/action",
                routing::post(act_on_table::<TableServices::ActOnTableServiceType>)
            )
            .with_state(self.table_services.act_on_table_service());
        info!("serving cardroom application ...");

        axum::serve(listener, router).await
    }
}


async fn create_tournament<Service: CreateTournament>(
    extract::State(service): extract::State<Service>,
    extract::Json(request): extract::Json<CreateTournamentRequest>,
) -> extract::Json<CreateTournamentResponse> {
    let auth_info = AuthInfo::new(Uuid::new_v4(), vec![AuthRole::Organizer]);
    let response = service.create_tournament(request, &auth_info).unwrap();
    extract::Json(response)
}


async fn act_on_table<Service: ActOnTable>(
    extract::State(service): extract::State<Service>,
    extract::Path(table_id): extract::Path<TableId>,
) {
    service.act_on_table(table_id, 42);
    // let auth_info = AuthInfo::new(Uuid::new_v4(), vec![AuthRole::Organizer]);
    // let response = service.create_tournament(request, &auth_info).unwrap();
    // extract::Json(response)
}
