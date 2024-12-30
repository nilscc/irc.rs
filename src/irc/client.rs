pub mod buffer;
pub mod capabilities;

#[cfg(test)]
mod test;

use buffer::Buffer;
use web_sys::WebSocket;
use yew::AttrValue;

use super::parser::Message;

pub trait Socket {
    type Error;

    fn new(host: &str) -> Result<Self, Self::Error>
    where
        Self: Sized;
}

impl Socket for WebSocket {
    type Error = ();
    fn new(host: &str) -> Result<Self, Error> {
        match WebSocket::new(host) {
            Ok(ws) => Ok(ws),
            Err(_) => Err(()),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Client {
    pub capabilities: Vec<Capability>,
    pub buffers: Vec<Buffer>,
}

// TODO: proper error type
type Error = ();

///
type Messages = Vec<Message>;

impl Client {
    pub fn new() -> Self {
        Client {
            capabilities: vec![],
            buffers: vec![],
        }
    }

    pub fn request_capabilities(_capabilities: Vec<Capability>) -> Result<Messages, Error> {
        Ok(vec![Message::cmd("CAP").param("LS").build()])
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Capability(pub AttrValue);
