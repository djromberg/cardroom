use std::collections::HashMap;

use log::info;
use uuid::Uuid;

use crate::domain::PublishTournamentMessages;
use crate::domain::SubscribeTableMessages;
use crate::domain::TournamentMessage;
use crate::domain::TournamentMessageType;


use crate::domain::TableMessage;

use tokio::sync::broadcast::Receiver;
use tokio::sync::broadcast::Sender;

pub type TableMessageReceiver = Receiver<TableMessage>;
pub type TableMessageSender = Sender<TableMessage>;



pub struct TableMessageBroadcast {
    senders: HashMap<(Uuid, usize), TableMessageSender>,
}

impl TableMessageBroadcast {
    pub fn new() -> Self {
        Self { senders: HashMap::new() }
    }
}


impl PublishTournamentMessages for TableMessageBroadcast {
    fn publish_tournament_messages(&self, messages: Vec<TournamentMessage>) {
        info!("publishing {:?}", messages);
        for tournament_message in messages {
            match tournament_message.message_type {
                TournamentMessageType::TableMessage { table_number, message_type } => {
                    let key = (tournament_message.tournament_id, table_number);
                    if let Some(sender) = self.senders.get(&key) {
                        _ = sender.send(message_type);
                    }
                }
            }
        }
    }
}


impl SubscribeTableMessages for TableMessageBroadcast {
    fn subscribe_table_messages(&mut self, tournament_id: Uuid, table_number: usize) -> TableMessageReceiver {
        let key = (tournament_id, table_number);

        if let Some(sender) = self.senders.get(&key) {
            sender.subscribe()
        } else {
            let sender = TableMessageSender::new(16);
            let receiver = sender.subscribe();
            self.senders.insert(key, sender);
            receiver
        }
    }
}
