use crate::domain::Bar;
use crate::domain::BarEvent;

use async_trait::async_trait;
use uuid::Uuid;


#[derive(Debug, Clone)]
pub struct BarOutboxEvent {
    pub id: Uuid,
    pub payload: BarEvent,
}


#[async_trait]
pub trait BarTransaction {
    async fn save_bar(&self, bar: Bar);
    async fn mark_event_processed(&self, event_id: Uuid, handler_name: &'static str);
    async fn commit(&self);
}


#[async_trait]
pub trait BarRepository {
    async fn create_transaction(&self) -> impl BarTransaction;
    async fn unpublished_events(&self) -> Vec<BarOutboxEvent>;
    async fn mark_published(&self, event_id: Uuid);
}


#[async_trait]
pub trait PublishBarEvents {
    async fn publish_events(&self, events: Vec<BarOutboxEvent>);
}
