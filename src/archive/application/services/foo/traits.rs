use crate::application::Commit;
use crate::application::MarkEventProcessed;
use crate::domain::Foo;
use crate::domain::FooEvent;

use async_trait::async_trait;
use uuid::Uuid;


#[derive(Debug, Clone)]
pub struct FooOutboxEvent {
    pub id: Uuid,
    pub payload: FooEvent,
}


#[async_trait]
pub trait SaveFoo {
    async fn save_foo(&mut self, foo: Foo);
}


#[async_trait]
pub trait FooTransaction: SaveFoo + MarkEventProcessed + Commit {}
impl<T: SaveFoo + MarkEventProcessed + Commit> FooTransaction for T {}


#[async_trait]
pub trait CreateFooTransaction {
    async fn create_transaction(&self) -> impl FooTransaction;
}


#[async_trait]
pub trait FooEventOutbox {
    async fn unpublished_events(&self) -> Vec<FooOutboxEvent>;
    async fn mark_event_published(&self, event_id: Uuid);
}


#[async_trait]
pub trait FooRepository: CreateFooTransaction + FooEventOutbox {}
impl<T: CreateFooTransaction + FooEventOutbox> FooRepository for T {}


#[async_trait]
pub trait PublishFooEvent {
    async fn publish_event(&self, event: FooOutboxEvent);
}
