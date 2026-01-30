use crate::application::AuthError;
use crate::application::AuthInfo;

use crate::domain::ReceiveTableEvent;
use crate::domain::RegisterForTableEvents;

use thiserror::Error;
use uuid::Uuid;


#[derive(Debug, Error)]
pub enum ObserveTableError {
    #[error(transparent)]
    AuthError(#[from] AuthError),
}


#[derive(Debug)]
pub struct ObserveTableRequest<Receiver: ReceiveTableEvent> {
    pub table_id: Uuid,
    pub receiver: Receiver,
}


#[derive(Debug)]
pub struct ObserveTableResponse {
}


pub trait ObserveTable {
    fn observe_table<Receiver: ReceiveTableEvent>(
        &mut self,
        request: ObserveTableRequest<Receiver>,
        auth_info: &AuthInfo
    ) -> Result<ObserveTableResponse, ObserveTableError>;
}


pub(in crate::application) fn observe_table<Receiver: ReceiveTableEvent, Broadcast: RegisterForTableEvents<Receiver=Receiver>>(
    request: ObserveTableRequest<Receiver>,
    auth_info: &AuthInfo,
    broadcast: &mut Broadcast,
) -> Result<ObserveTableResponse, ObserveTableError> {
    // TODO: Think about whether public / unauthenticated observation should
    //       also be handled here. We do not want to duplicate service code.
    //       An authenticated request whose author sits at the table could
    //       receive private events.
    _ = auth_info.ensure_authenticated()?;
    broadcast.register_for_table_events(request.table_id, request.receiver);
    Ok(ObserveTableResponse {})
}


#[cfg(test)]
mod tests {
    use std::{cell::Cell, collections::HashMap};

    use crate::{application::auth::AuthRole, domain::{TableEvent, TableEventType}};

    use super::*;

    struct DummyReceiver {
        events: Cell<Vec<TableEvent>>,
    }

    impl DummyReceiver {
        fn new() -> Self {
            Self { events: Cell::new(vec![]) }
        }

        fn consume(&self) -> Vec<TableEvent> {
            self.events.take()
        }
    }

    impl ReceiveTableEvent for DummyReceiver {
        fn receive_table_event(&self, event: TableEvent) {
            let mut current_events = self.events.take();
            current_events.push(event);
            self.events.set(current_events);
        }
    }


    struct DummyBroadcast {
        receivers: HashMap<Uuid, DummyReceiver>,
    }

    impl DummyBroadcast {
        fn new() -> Self {
            Self { receivers: HashMap::new() }
        }

        fn receiver(&self, table_id: Uuid) -> Option<&DummyReceiver> {
            self.receivers.get(&table_id)
        }

        fn send(&self, event: TableEvent) {
            let event_table_id = event.table_id;
            for (table_id, receiver) in &self.receivers {
                if table_id == &event_table_id {
                    receiver.receive_table_event(event.clone());
                }
            }
        }
    }

    impl RegisterForTableEvents for DummyBroadcast {
        type Receiver = DummyReceiver;

        fn register_for_table_events(&mut self, table_id: Uuid, receiver: Self::Receiver) {
            let real_receiver = DummyReceiver::new();
            self.receivers.insert(table_id, real_receiver);
        }
    }

    #[test]
    fn observe_table_dummy() {
        let mut broadcast = DummyBroadcast::new();
        let receiver = DummyReceiver::new();
        let auth_info = AuthInfo::Authenticated { account_id: Uuid::new_v4(), role: AuthRole::Member };
        let table_id = Uuid::new_v4();
        let event = TableEvent { table_id, event_type: TableEventType::PlayerLeft { position: 0 } };
        let request = ObserveTableRequest { table_id, receiver };
        let result = observe_table(request, &auth_info, &mut broadcast);
        assert!(result.is_ok());
        broadcast.send(event.clone());
        let receiver = broadcast.receiver(table_id);
        assert!(receiver.is_some_and(|receiver| receiver.consume() == vec![event]));
    }
}
