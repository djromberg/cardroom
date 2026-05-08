use std::sync::Arc;

use crate::application::AuthInfo;
use crate::application::ApplicationError;
use crate::application::EventBus;
use crate::domain::TournamentEvent;

use async_trait::async_trait;
use tokio::sync::Mutex;
use uuid::Uuid;


#[async_trait]
pub trait DoStuff {
    async fn do_stuff(&self, do_it_cool: bool, auth_info: &AuthInfo) -> Result<Uuid, ApplicationError>;
}


#[derive(Debug, Clone)]
pub struct DoStuffService {
    resource: Arc<Mutex<Uuid>>,
    event_bus: EventBus<TournamentEvent>,
}

impl DoStuffService {
    pub fn new(resource: Arc<Mutex<Uuid>>, event_bus: EventBus<TournamentEvent>) -> Self {
        Self { resource, event_bus }
    }
}

#[async_trait]
impl DoStuff for DoStuffService {
    async fn do_stuff(&self, do_it_cool: bool, auth_info: &AuthInfo) -> Result<Uuid, ApplicationError> {
        let account_id = auth_info.expect_organizer()?;
        let mut resource = self.resource.lock().await;
        *resource = account_id;
        log::info!("Changed to {}", *resource);
        self.event_bus.send(vec![]);
        Ok(*resource)
    }
}
