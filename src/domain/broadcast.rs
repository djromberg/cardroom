use std::collections::HashMap;

use log::info;
use uuid::Uuid;

use crate::domain::RegisterTableEventReceivers;
use crate::domain::PublishTournamentEvents;
use crate::domain::SubscribeTableEvents;
use crate::domain::TournamentEvent;
use crate::domain::TournamentEventType;


use crate::domain::TableEvent;

use tokio::sync::broadcast::Receiver;
use tokio::sync::broadcast::Sender;

pub type TableEventReceiver = Receiver<TableEvent>;
pub type TableEventSender = Sender<TableEvent>;



pub struct TableEventBroadcast {
    senders: HashMap<(Uuid, usize), TableEventSender>,
}

impl TableEventBroadcast {
    pub fn new() -> Self {
        Self { senders: HashMap::new() }
    }
}


impl RegisterTableEventReceivers for TableEventBroadcast {
    fn register_table_event_receivers(&mut self, tournament_id: Uuid, table_count: usize) {
        for i in 0..table_count {
            let key = (tournament_id, i);
            let sender = TableEventSender::new(16);
            let old = self.senders.insert(key, sender);
            assert!(old.is_none());
        }
    }
}


impl PublishTournamentEvents for TableEventBroadcast {
    fn publish_tournament_events(&self, events: Vec<TournamentEvent>) {
        info!("publishing {:?}", events);
        for tournament_event in events {
            match tournament_event.event_type {
                TournamentEventType::TableEvent { table_number, event_type } => {
                    let key = (tournament_event.tournament_id, table_number);
                    if let Some(sender) = self.senders.get(&key) {
                        _ = sender.send(event_type);
                    }
                }
            }
        }
    }
}


impl SubscribeTableEvents for TableEventBroadcast {
    fn subscribe_table_events(&self, tournament_id: Uuid, table_number: usize) -> Option<TableEventReceiver> {
        let key = (tournament_id, table_number);

        if let Some(sender) = self.senders.get(&key) {
            Some(sender.subscribe())
        } else {
            None
        }
    }
}
