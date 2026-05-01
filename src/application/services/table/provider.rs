use crate::application::ActOnTable;
use crate::application::ActOnTableService;
use crate::application::EventBus;
use crate::application::OpenTables;
use crate::application::OpenTablesService;
use crate::application::TableRepository;

use crate::domain::TableEvent;


pub trait ProvideTableServices {
    type ActOnTableServiceType: ActOnTable + Clone + Send + Sync + 'static;

    fn act_on_table_service(&self) -> Self::ActOnTableServiceType;
}


pub trait ProvidePrivateTableServices {
    type OpenTablesServiceType: OpenTables + Clone + Send + Sync + 'static;

    fn open_tables_service(&self) -> Self::OpenTablesServiceType;
}


#[derive(Debug, Clone)]
pub struct TableServiceProvider<Repository> {
    repository: Repository,
    event_bus: EventBus<TableEvent>,
}

impl<Repository> TableServiceProvider<Repository> {
    pub fn new(repository: Repository) -> Self {
        Self { repository, event_bus: EventBus::new() }
    }
}


impl<Repository: TableRepository> ProvideTableServices for TableServiceProvider<Repository> {
    type ActOnTableServiceType = ActOnTableService<Repository>;

    fn act_on_table_service(&self) -> Self::ActOnTableServiceType {
        ActOnTableService::new(self.repository.clone(), self.event_bus.clone())
    }
}



impl<Repository: TableRepository> ProvidePrivateTableServices for TableServiceProvider<Repository> {
    type OpenTablesServiceType = OpenTablesService<Repository>;

    fn open_tables_service(&self) -> Self::OpenTablesServiceType {
        OpenTablesService::new(self.repository.clone(), self.event_bus.clone())
    }
}
