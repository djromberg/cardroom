use crate::{application::{Commit, CreateFooTransaction, FooEventOutbox, FooOutboxEvent, FooRepository, FooTransaction, MarkEventProcessed, SaveFoo}, domain::{Foo, FooEvent}};

use async_trait::async_trait;
use tokio::sync::Mutex;
use uuid::Uuid;

use std::{collections::{HashMap, HashSet}, sync::Arc};


#[derive(Debug)]
struct FooDatabase {
    foos: HashMap<Uuid, Foo>,
    unpublished_events: Vec<FooOutboxEvent>,
    processed_events: HashMap<String, HashSet<Uuid>>,
}

impl FooDatabase {
    fn new() -> Self {
        Self {
            foos: HashMap::new(),
            unpublished_events: vec![],
            processed_events: HashMap::new()
        }
    }

    fn remove_unpublished_event(&mut self, event_id: Uuid) {
        if let Some(index) = self.unpublished_events.iter().position(|event| event.id == event_id) {
            self.unpublished_events.remove(index);
        }
    }
}


#[derive(Debug)]
struct InMemoryFooTransaction {
    db: Arc<Mutex<FooDatabase>>,
    saved_foos: Vec<Foo>,
    events_processed: Vec<(Uuid, String)>,
}

impl InMemoryFooTransaction {
    fn new(db: Arc<Mutex<FooDatabase>>) -> Self {
        Self { db, saved_foos: vec![], events_processed: vec![] }
    }
}

#[async_trait]
impl SaveFoo for InMemoryFooTransaction {
    async fn save_foo(&mut self, foo: Foo) {
        self.saved_foos.push(foo);
    }
}

#[async_trait]
impl MarkEventProcessed for InMemoryFooTransaction {
    async fn mark_event_processed(&mut self, event_id: Uuid, handler_name: &'static str) {
        self.events_processed.push((event_id, handler_name.to_string()))
    }
}

#[async_trait]
impl Commit for InMemoryFooTransaction {
    async fn commit(&mut self) {
        let mut db = self.db.lock().await;
        for foo in &mut self.saved_foos {
            let events = foo.consume_events();
            let outbox_events = events.iter().map(|event| FooOutboxEvent {id: Uuid::new_v4(), payload: event.clone()});
            db.unpublished_events.extend(outbox_events);
            db.foos.insert(foo.id(), foo.clone());
        }
        self.saved_foos = vec![];
    }
}


#[derive(Debug, Clone)]
struct InMemoryFooRepository {
    db: Arc<Mutex<FooDatabase>>,
}

impl InMemoryFooRepository {
    fn new() -> Self {
        Self { db: Arc::new(Mutex::new(FooDatabase::new())) }
    }
}

#[async_trait]
impl CreateFooTransaction for InMemoryFooRepository {
    async fn create_transaction(&self) -> InMemoryFooTransaction {
        InMemoryFooTransaction::new(self.db.clone())
    }
}

#[async_trait]
impl FooEventOutbox for InMemoryFooRepository {
    async fn unpublished_events(&self) -> Vec<FooOutboxEvent> {
        let db = self.db.lock().await;
        db.unpublished_events.clone()
    }

    async fn mark_event_published(&self, event_id: Uuid) {
        let mut db = self.db.lock().await;
        db.remove_unpublished_event(event_id);
    }
}
