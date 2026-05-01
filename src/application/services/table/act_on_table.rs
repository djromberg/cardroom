use uuid::Uuid;

use crate::application::ApplicationError;
use crate::application::EventBus;
use crate::application::TableRepository;

use crate::domain::PlayerId;
use crate::domain::TableEvent;
use crate::domain::TableId;


#[derive(Debug)]
pub enum TableAction {
    Check,
    Bet(u32),
    Fold,
}


pub trait ActOnTable {
    fn act_on_table(&self, table_id: Uuid, action: TableAction) -> Result<(), ApplicationError>;
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

}

impl<Repository: TableRepository> ActOnTable for ActOnTableService<Repository> {
    fn act_on_table(&self, table_id: Uuid, action: TableAction) -> Result<(), ApplicationError> {
        log::info!("Trying to act on table {:?} with action {:?}", table_id, action);
        let events = self.repository.with_tx(|tx| {
            let mut table = tx.load_table(TableId::from_uuid(table_id))?;
            table.check(PlayerId::new_v4())?;
            tx.save_table(table)?;
            Ok(())
        })?;
        log::info!("Player acted on table {:?} with action {:?}", table_id, action);
        self.event_bus.send(events);
        Ok(())
    }
}
