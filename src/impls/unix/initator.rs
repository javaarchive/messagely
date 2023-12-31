use async_trait::async_trait;

use crate::network::{context::ManagerContext, initiator::Initiator, connection::Connection};

use super::transport::UnixTransport;

struct UnixInitator {
    pub path: String,
    pub context: ManagerContext
}

#[async_trait]
impl Initiator for UnixInitator {
    
    fn init(&mut self) {

        let path = self.path.clone();

        let context = self.context.clone();

        self.context.runtime.spawn(async move {
            let listener = tokio::net::UnixListener::bind(path).unwrap();
            loop {
                let (mut socket, _) = listener.accept().await.unwrap();
                // build new connection and add to connections
                let conn = Connection::new(Box::new(UnixTransport::new(socket)), context.clone());
                context.connections.lock().unwrap().push(conn);
            }
        });
    }
}