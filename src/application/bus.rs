use tokio::sync::broadcast::{Receiver, Sender};


#[derive(Debug, Clone)]
pub struct EventBus<EventType> {
    sender: Sender<EventType>,
}

impl<EventType> EventBus<EventType> {
    pub fn new() -> Self {
        Self { sender: Sender::new(u16::MAX as usize) }
    }

    pub fn subscribe(&self) -> Receiver<EventType> {
        self.sender.subscribe()
    }

    pub fn send(&self, events: Vec<EventType>) {
        for event in events {
            self.sender.send(event);
            // TODO: handle send error
        }
    }
}
