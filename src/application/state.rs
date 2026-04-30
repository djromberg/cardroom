use std::sync::mpsc::channel;
use std::sync::mpsc::Sender;
use std::sync::mpsc::Receiver;

use crate::application::EventBus;


#[derive(Debug)]
pub struct ApplicationState<Repository, EventType> {
    repository: Repository,
    sender: Sender<EventType>,
    receiver: Receiver<EventType>,
}

impl<Repository: Clone, EventType> ApplicationState<Repository, EventType> {
    pub fn new(repository: Repository) -> Self {
        let (sender, receiver) = channel();
        Self { repository, sender, receiver }
    }

    pub fn repository(&self) -> Repository {
        self.repository.clone()
    }

    pub fn event_bus(&self) -> EventBus<EventType> {
        EventBus::new(self.sender.clone())
    }

    pub fn receive_event(&self) -> Option<EventType> {
        if let Ok(event) = self.receiver.try_recv() {
            Some(event)
        } else {
            None
        }
    }
}
