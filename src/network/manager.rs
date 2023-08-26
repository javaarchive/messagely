
use std::sync::{Mutex, Arc};

use super::initiator::Initiator;

use super::{initiator, connection::Connection, context::ManagerContext};

pub struct Manager {
    pub initiators: Vec<Box<dyn Initiator>>,
    pub runtime: Option<Arc<tokio::runtime::Runtime>>,
    pub connections: Arc<Mutex<Vec<Connection>>>
}

impl Manager {

    pub fn init(&mut self){
        // setup async runtime
        self.runtime = Some(Arc::new(tokio::runtime::Runtime::new().unwrap()));
        // start internal loop for everything
        for initiator in &mut self.initiators {
            initiator.init();
        }
    }

    pub fn new(optional_initators: Option<Vec<Box<dyn Initiator>>>) -> Self {
        let mut manager = Manager {
            initiators: Vec::new(),
            runtime: None,
            connections: Arc::new(Mutex::new(Vec::new()))
        };

        if let Some(initiators) = optional_initators {
            manager.initiators = initiators;
        }

        manager.init();
        manager
    }

    pub fn get_context(&self) -> ManagerContext {
        ManagerContext {
            runtime: self.runtime.as_ref().unwrap().clone(),
            connections: self.connections.clone() 
        }
    }
}