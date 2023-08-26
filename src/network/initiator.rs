// a server thing that makes new connections

use async_trait::async_trait;

pub trait Initiator {
    fn init(&mut self);
}