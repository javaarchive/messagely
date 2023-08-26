use async_trait::async_trait;
use tokio::{net::UnixStream, io::{Interest, AsyncReadExt, AsyncWriteExt}};

use crate::network::transport::Transport;

struct UnixTransport {
    socket: UnixStream,
}

impl UnixTransport {
    pub fn new(socket: UnixStream) -> Self {
        Self { socket }
    }
}

#[async_trait]
impl Transport for UnixTransport {
    async fn setup(&mut self) -> std::io::Result<()> {
        self.socket.ready(Interest::READABLE | Interest::WRITABLE).await?;
        Ok(())
    }

    async fn read_exact(&mut self, buf: &mut [u8]) -> std::io::Result<()> {
        self.socket.read_exact(buf).await?;
        Ok(())
    }

    async fn send_vec(&mut self, buf: &Vec<u8>) -> std::io::Result<()> {
        self.socket.write_all(buf).await?;
        Ok(())
    }
}