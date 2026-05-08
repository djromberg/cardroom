use crate::application::AccessStuff;
use crate::application::RepositoryError;
use crate::application::Stuff;

use async_trait::async_trait;
use tokio::sync::Mutex;
use uuid::Uuid;

use std::sync::Arc;


#[derive(Debug, Clone)]
struct Resource;


impl Resource {
    fn new() -> Self {
        Self
    }

    fn load(&self, stuff_id: Uuid) -> Stuff {
        Stuff::new(stuff_id)
    }

    fn save(&mut self, stuff: Stuff) {
    }
}


#[derive(Debug, Clone)]
pub struct InMemoryResourceAccessor {
    resource: Arc<Mutex<Resource>>,
}

impl InMemoryResourceAccessor {
    pub fn new() -> Self {
        Self { resource: Arc::new(Mutex::new(Resource::new())) }
    }
}


#[async_trait]
impl AccessStuff for InMemoryResourceAccessor {
    async fn access_stuff(&self, stuff_id: Uuid) -> Result<Stuff, RepositoryError> {
        let resource = self.resource.lock().await;
        let stuff = resource.load(stuff_id);
        Ok(stuff)
    }
}
