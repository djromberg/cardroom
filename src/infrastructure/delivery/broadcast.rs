use std::collections::HashMap;

use log::info;
use uuid::Uuid;

use crate::domain::PublishTableEvents;
use crate::domain::ReceiveTableEvent;
use crate::domain::RegisterForTableEvents;
use crate::domain::TableEvent;


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
    receivers: HashMap<Uuid, Vec<LoggingReceiver>>,
}

impl LoggingBroadcast {
    pub fn new() -> Self {
        Self { receivers: HashMap::new() }
    }
}


impl PublishTableEvents for LoggingBroadcast {
    fn publish_table_events(&self, events: Vec<TableEvent>) {
        info!("publishing {:?}", events);
        for event in events {
            for (table_id, receivers) in &self.receivers {
                if table_id == &event.table_id {
                    for receiver in receivers {
                        receiver.receive_table_event(event.clone());
                    }
                }
            }
        }
    }
}


impl RegisterForTableEvents for LoggingBroadcast {
    type Receiver = LoggingReceiver;

    fn register_for_table_events(&mut self, table_id: uuid::Uuid, receiver: Self::Receiver) {
        if let Some(receivers) = self.receivers.get_mut(&table_id) {
            receivers.push(receiver);
        } else {
            self.receivers.insert(table_id, vec![receiver]);
        }
    }
}
