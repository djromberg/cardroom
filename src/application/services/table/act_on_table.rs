use crate::application::ApplicationError;
use crate::application::AuthInfo;
use crate::application::EventBus;
use crate::application::TableRepository;

use crate::domain::TableAction;
use crate::domain::TableEvent;
use crate::domain::TableId;

use uuid::Uuid;


pub trait ActOnTable {
    fn check(&self, table_id: Uuid, auth_info: &AuthInfo) -> Result<(), ApplicationError>;
    fn bet(&self, table_id: Uuid, amount: u32, auth_info: &AuthInfo) -> Result<(), ApplicationError>;
    fn fold(&self, table_id: Uuid, auth_info: &AuthInfo) -> Result<(), ApplicationError>;
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

    fn act_on_table(&self, table_id: TableId, action: TableAction, auth_info: &AuthInfo) -> Result<(), ApplicationError> {
        let player_id = auth_info.expect_participant()?;
        log::info!("Player {:} is trying to act on table {:?} with action {:?}", player_id, table_id, action);
        let events = self.repository.with_tx(|tx| {
            let mut table = tx.load_table(table_id)?;
            table.act(player_id, action)?;
            tx.save_table(table)?;
            Ok(())
        })?;
        log::info!("Player acted on table {:?} with action {:?}", table_id, action);
        self.event_bus.send(events);
        Ok(())
    }
}

impl<Repository: TableRepository> ActOnTable for ActOnTableService<Repository> {
    fn bet(&self, table_id: Uuid, amount: u32, auth_info: &AuthInfo) -> Result<(), ApplicationError> {
        self.act_on_table(TableId::from_uuid(table_id), TableAction::Bet(amount), auth_info)
    }

    fn check(&self, table_id: Uuid, auth_info: &AuthInfo) -> Result<(), ApplicationError> {
        self.act_on_table(TableId::from_uuid(table_id), TableAction::Check, auth_info)
    }

    fn fold(&self, table_id: Uuid, auth_info: &AuthInfo) -> Result<(), ApplicationError> {
        self.act_on_table(TableId::from_uuid(table_id), TableAction::Fold, auth_info)
    }
}
