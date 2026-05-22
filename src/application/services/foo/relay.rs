use crate::application::FooEventOutbox;
use crate::application::PublishFooEvent;


pub async fn relay_foo_events<Outbox: FooEventOutbox, EventBus: PublishFooEvent>(outbox: &Outbox, event_bus: &EventBus) {
    loop {
        let events = outbox.unpublished_events().await;
        for event in events {
            let event_id = event.id;
            event_bus.publish_event(event).await;
            outbox.mark_event_published(event_id).await;
        }
    }
}
