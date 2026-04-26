use std::sync::mpsc::Receiver;
use std::sync::mpsc::Sender;

use crate::domain::TournamentEvent;

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

pub type TournamentEventBus = EventBus<TournamentEvent>;
pub type TournamentEventReceiver = Receiver<TournamentEvent>;
