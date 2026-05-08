use crate::application::AuthInfo;
use crate::application::ApplicationError;
use crate::application::EventBus;
use crate::application::RepositoryError;
use crate::domain::TournamentEvent;

use async_trait::async_trait;
use uuid::Uuid;


#[derive(Debug, Clone)]
pub struct Stuff {
    id: Uuid
}

impl Stuff {
    pub fn new(id: Uuid) -> Self {
        Self { id }
    }

    pub fn mutate(&mut self) {
    }
}


#[async_trait]
pub trait AccessStuff: Clone + Sync + Send + 'static {
    async fn access_stuff(&self, stuff_id: Uuid) -> Result<Stuff, RepositoryError>;
}


#[async_trait]
pub trait DoStuff: Clone + Send + Sync + 'static {
    async fn do_stuff(&self, do_it_cool: bool, auth_info: &AuthInfo) -> Result<Uuid, ApplicationError>;
}


#[derive(Debug, Clone)]
pub struct DoStuffService<Repository> {
    repository: Repository,
    event_bus: EventBus<TournamentEvent>,
}

impl<Repository> DoStuffService<Repository> {
    pub fn new(repository: Repository, event_bus: EventBus<TournamentEvent>) -> Self {
        Self { repository, event_bus }
    }
}

#[async_trait]
impl<Repository: AccessStuff> DoStuff for DoStuffService<Repository> {
    async fn do_stuff(&self, do_it_cool: bool, auth_info: &AuthInfo) -> Result<Uuid, ApplicationError> {
        let account_id = auth_info.expect_organizer()?;
        let mut stuff = self.repository.access_stuff(account_id).await?;
        stuff.mutate();
        self.event_bus.send(vec![]);
        Ok(account_id)
    }
}
