pub mod buffer;
pub mod capabilities;

#[cfg(test)]
mod test;

use buffer::Buffer;
use capabilities::{CapNegotiator, Capability};

use super::parser::Message;

#[derive(Debug, PartialEq, Clone)]
enum ClientState {
    PreCapLs,
    CapLs,
    //CapReq,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Error {}

#[derive(Debug, Clone, PartialEq)]
pub struct Client {
    state: ClientState,
    cap_negotiator: CapNegotiator,
    pub buffers: Vec<Buffer>,
}

impl Client {
    // Supported list of capabilities
    pub fn supported_capabilities() -> Vec<Capability> {
        vec![Capability::new("sasl")]
    }

    pub fn new() -> Self {
        Client {
            state: ClientState::PreCapLs,
            cap_negotiator: CapNegotiator::request(Self::supported_capabilities()),
            buffers: vec![],
        }
    }

    pub fn request_capabilities(&mut self) -> Message {
        self.state = ClientState::CapLs;
        self.cap_negotiator.ls(Some("302"))
    }

    pub fn handle(&mut self, _message: &Message) -> Result<(), Error> {
        Ok(())
    }
}
