use super::parser::Message;

pub mod capabilities;

#[cfg(test)]
mod test;

pub struct Client {
    pub capabilities: Vec<Capability>,
    pub buffers: Vec<Buffer>,
}

// TODO: proper error type
type Error = ();

///
type Messages = Vec<Message>;

impl Client {
    pub fn request_capabilities(_capabilities: Vec<Capability>) -> Result<Messages, Error> {
        Ok(vec![Message::cmd("CAP").param("LS").build()])
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Capability(pub String);

pub struct Buffer {}
