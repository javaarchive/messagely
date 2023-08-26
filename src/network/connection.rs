use std::sync::{Arc, Mutex};

use super::{transport::Transport, context::ManagerContext};

pub struct Connection {
    pub transport: Arc<Mutex<Box<dyn Transport + Send>>>,
    pub active: bool,
    pub context: ManagerContext
}

impl Connection {
    pub fn new(transport: Box<dyn Transport + Send>, context: ManagerContext) -> Self {
        Connection {
            transport: Arc::new(Mutex::new(transport)),
            active: false,
            context
        }
    }

    pub async fn setup(&mut self) -> std::io::Result<()> {
        self.active = true;
        self.transport.lock().unwrap().setup().await?;
        let transport_read_arc = self.transport.clone();
        self.context.runtime.spawn(async move {
            loop {
                
            }
        });
        Ok(())
    }

    pub async fn is_active(&self) -> bool {
        self.active
    }
}