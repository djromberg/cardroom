use std::collections::HashMap;

use log::info;
use uuid::Uuid;

use crate::domain::PublishTournamentEvents;
use crate::domain::ReceiveTableEvent;
use crate::domain::RegisterForTableEvents;
use crate::domain::TableEvent;
use crate::domain::TournamentEvent;
use crate::domain::TournamentEventType;


pub struct LoggingReceiver {
}

impl LoggingReceiver {
    pub fn new() -> Self {
        Self {}
    }
}

impl ReceiveTableEvent for LoggingReceiver {
    fn receive_table_event(&self, event: TableEvent) {
        info!("{:?}", event)
    }
}





pub struct LoggingBroadcast {
    receivers: HashMap<(Uuid, usize), Vec<LoggingReceiver>>,
}

impl LoggingBroadcast {
    pub fn new() -> Self {
        Self { receivers: HashMap::new() }
    }
}


impl PublishTournamentEvents for LoggingBroadcast {
    fn publish_tournament_events(&self, events: Vec<TournamentEvent>) {
        info!("publishing {:?}", events);
        for tournament_event in events {
            match tournament_event.event_type {
                TournamentEventType::TableEvent { table_number, event_type } => {
                    for ((tournament_id, recv_table_number), receivers) in &self.receivers {
                        if tournament_id == &tournament_event.tournament_id && table_number == *recv_table_number {
                            for receiver in receivers {
                                receiver.receive_table_event(event_type.clone());
                            }
                        }
                    }
                }
            }
        }
    }
}


impl RegisterForTableEvents for LoggingBroadcast {
    type Receiver = LoggingReceiver;

    fn register_for_table_events(&mut self, tournament_id: Uuid, table_number: usize, receiver: Self::Receiver) {
        let key = (tournament_id, table_number);
        if let Some(receivers) = self.receivers.get_mut(&key) {
            receivers.push(receiver);
        } else {
            self.receivers.insert(key, vec![receiver]);
        }
    }
}
