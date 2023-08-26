use std::sync::{Arc, Mutex};

use super::connection::Connection;

#[derive(Clone)]
pub struct ManagerContext {
    pub runtime: Arc<tokio::runtime::Runtime>,
    pub connections: Arc<Mutex<Vec<Connection>>>
}