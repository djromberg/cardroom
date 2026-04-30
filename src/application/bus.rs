use std::sync::mpsc::Sender;


#[derive(Debug, Clone)]
pub struct EventBus<Event> {
    sender: Sender<Event>,
}

impl<E> EventBus<E> {
    pub fn new(sender: Sender<E>) -> Self {
        Self { sender }
    }

    pub fn send(&self, events: Vec<E>) {
        for event in events {
            self.sender.send(event);
        }
    }
}
