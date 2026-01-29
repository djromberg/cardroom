use log::info;

use crate::domain::PublishTableEvents;
use crate::domain::TableEvent;


#[derive(Debug)]
pub struct DummyBroadcast {
}

impl DummyBroadcast {
    pub fn new() -> Self {
        Self { }
    }
}


impl PublishTableEvents for DummyBroadcast {
    fn publish_table_events(&self, events: Vec<TableEvent>) {
        info!("publishing {:?}", events);
    }
}
