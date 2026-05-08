use crate::application::ApplicationError;
use crate::application::AuthInfo;
use crate::application::EventBus;
use crate::application::TableRepository;

use crate::domain::TableAction;
use crate::domain::TableEvent;
use crate::domain::TableId;

use async_trait::async_trait;
use uuid::Uuid;


#[async_trait]
pub trait ActOnTable {
    async fn check(&self, table_id: Uuid, auth_info: &AuthInfo) -> Result<(), ApplicationError>;
    async fn bet(&self, table_id: Uuid, amount: u32, auth_info: &AuthInfo) -> Result<(), ApplicationError>;
    async fn fold(&self, table_id: Uuid, auth_info: &AuthInfo) -> Result<(), ApplicationError>;
}


#[derive(Debug, Clone)]
pub struct ActOnTableService<Repository> {
    repository: Repository,
    event_bus: EventBus<TableEvent>,
}

impl<Repository: TableRepository> ActOnTableService<Repository> {
    pub fn new(repository: Repository, event_bus: EventBus<TableEvent>) -> Self {
        Self { repository, event_bus }
    }

    async fn act_on_table(&self, table_id: TableId, action: TableAction, auth_info: &AuthInfo) -> Result<(), ApplicationError> {
        let player_id = auth_info.expect_participant()?;
        log::info!("Player {:} is trying to act on table {:?} with action {:?}", player_id, table_id, action);
        let mut table = self.repository.load_table(table_id).await?;
        table.act(player_id, action)?;
        let events = self.repository.save_table(table).await?;
        log::info!("Player acted on table {:?} with action {:?}", table_id, action);
        self.event_bus.send(events);
        Ok(())
    }
}

#[async_trait]
impl<Repository: TableRepository> ActOnTable for ActOnTableService<Repository> {
    async fn bet(&self, table_id: Uuid, amount: u32, auth_info: &AuthInfo) -> Result<(), ApplicationError> {
        self.act_on_table(TableId::from_uuid(table_id), TableAction::Bet(amount), auth_info).await
    }

    async fn check(&self, table_id: Uuid, auth_info: &AuthInfo) -> Result<(), ApplicationError> {
        self.act_on_table(TableId::from_uuid(table_id), TableAction::Check, auth_info).await
    }

    async fn fold(&self, table_id: Uuid, auth_info: &AuthInfo) -> Result<(), ApplicationError> {
        self.act_on_table(TableId::from_uuid(table_id), TableAction::Fold, auth_info).await
    }
}
