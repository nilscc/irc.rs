use crate::irc::parser::{Command, Message, MessageBuilderError};

use super::Capability;

#[cfg(test)]
mod test;

type Result = std::result::Result<Messages, Error>;
type Messages = Vec<Message>;

#[derive(Debug)]
pub enum Error {
    UnexpectedCommand(Command),
    MessageBuilderError(MessageBuilderError),
}

pub struct CapNegotiator {
    _requested: Vec<Capability>,
}

impl CapNegotiator {
    pub fn request(capabilities: Vec<Capability>) -> Self {
        CapNegotiator {
            _requested: capabilities,
        }
    }

    pub fn ls(&self, version: Option<String>) -> Message {
        let mut builder = Message::cmd("CAP").param("LS");

        if let Some(version) = version {
            builder = builder.param(version.as_ref());
        }

        builder.build().unwrap()
    }

    pub fn handle(&mut self, message: Message) -> Result {
        if Command::Cmd("CAP".into()) != message.command {
            return Err(Error::UnexpectedCommand(message.command));
        }

        let nick = &message.parameters[0];
        let subcmd = &message.parameters[1];
        let param = &message.parameters[2..];

        match subcmd.as_ref() {
            "LS" => {}
            _ => {}
        }

        println!("{nick:?} {subcmd:?} {param:?}");

        Ok(vec![])
    }
}
