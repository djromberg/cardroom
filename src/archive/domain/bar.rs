use uuid::Uuid;


#[derive(Debug, Clone)]
pub enum BarEvent {
    BarCreated {
        bar_id: Uuid,
        foo_id: Uuid,
    },
    BarDestroyed {
        bar_id: Uuid,
        foo_id: Uuid,
    },
}


#[derive(Debug, Clone)]
pub struct Bar {
    id: Uuid,
    foo_id: Uuid,
    is_destroyed: bool,
    events: Vec<BarEvent>,
}

impl Bar {
    pub fn new(id: Uuid, foo_id: Uuid) -> Self {
        let mut bar = Self { id, foo_id, is_destroyed: false, events: vec![] };
        bar.record_event(BarEvent::BarCreated { bar_id: id, foo_id });
        bar
    }

    pub fn destroy(&mut self) {
        assert!(!self.is_destroyed);
        self.is_destroyed = true;
        self.record_event(BarEvent::BarDestroyed { bar_id: self.id, foo_id: self.foo_id });
    }

    pub fn consume_events(&mut self) -> Vec<BarEvent> {
        std::mem::take(&mut self.events)
    }

    fn record_event(&mut self, event: BarEvent) {
        self.events.push(event);
    }
}
