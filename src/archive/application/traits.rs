use async_trait::async_trait;
use uuid::Uuid;


#[async_trait]
pub trait MarkEventProcessed {
    async fn mark_event_processed(&mut self, event_id: Uuid, handler_name: &'static str);
}


#[async_trait]
pub trait Commit {
    async fn commit(&mut self);
}
