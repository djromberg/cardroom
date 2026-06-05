use uuid::Uuid;


#[derive(Debug, Clone)]
pub enum FooEvent {
    FooCreated {
        foo_id: Uuid,
        bar_count: u16,
    },
    BarAdded {
        foo_id: Uuid,
        bar_id: Uuid,
    },
    BarRemoved {
        foo_id: Uuid,
        bar_id: Uuid,
    }
}


#[derive(Debug, Clone)]
pub struct Foo {
    id: Uuid,
    bar_count: u16,
    bar_ids: Vec<Uuid>,
    events: Vec<FooEvent>,
}

impl Foo {
    pub fn new(id: Uuid, bar_count: u16) -> Self {
        let mut foo = Self { id, bar_count, bar_ids: vec![], events: vec![] };
        foo.record_event(FooEvent::FooCreated { foo_id: id, bar_count });
        foo
    }

    pub fn id(&self) -> Uuid {
        self.id
    }

    pub fn add_bar(&mut self, bar_id: Uuid) {
        assert!(self.bar_ids.len() < self.bar_count as usize);
        assert!(!self.bar_ids.contains(&bar_id));
        self.bar_ids.push(bar_id);
        self.record_event(FooEvent::BarAdded { foo_id: self.id, bar_id });
    }

    pub fn remove_bar(&mut self, bar_id: Uuid) {
        let index = self.bar_ids.iter().position(|bar_id| bar_id == bar_id).unwrap();
        self.bar_ids.remove(index);
        self.record_event(FooEvent::BarRemoved { foo_id: self.id, bar_id });
    }

    pub fn consume_events(&mut self) -> Vec<FooEvent> {
        std::mem::take(&mut self.events)
    }

    fn record_event(&mut self, event: FooEvent) {
        self.events.push(event);
    }
}
