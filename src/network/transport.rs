
use async_trait::async_trait;

#[async_trait]
pub trait Transport {
    async fn setup(&mut self) -> std::io::Result<()> {
        println!("Transport default unimplemented setup.");
        Ok(())
    }

    async fn read_exact(&mut self, buf: &mut [u8]) -> std::io::Result<()> {
        println!("Transport default unimplemented read_exact.");
        Ok(())
    }

    async fn send_vec(&mut self, buf: &Vec<u8>) -> std::io::Result<()> {
        println!("Transport default unimplemented send_vec.");
        Ok(())
    }
}