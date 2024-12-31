use yew::AttrValue;

/// IRCv3 Capability negotiation, following the spec:
///
/// https://ircv3.net/specs/extensions/capability-negotiation.html
use crate::irc::parser::{Command, Message, MessageBuilderError};

#[cfg(test)]
mod test;

type Result = std::result::Result<Messages, Error>;
type Messages = Vec<Message>;

#[derive(Debug, Clone, PartialEq)]
pub struct Capability(pub AttrValue);

impl Capability {
    pub fn new(value: &str) -> Self {
        Capability(value.to_owned().into())
    }
}

#[derive(Debug)]
pub enum Error {
    UnexpectedCommand(Command),
    UnexpectedSubcommand(String),
    MessageBuilderError(MessageBuilderError),
}

#[derive(Debug, Clone, PartialEq)]
pub struct CapNegotiator {
    pub requested: Vec<Capability>,
    pub acknowledged: Vec<Capability>,
    pub not_acknowledged: Vec<Capability>,
}

impl CapNegotiator {
    pub fn request(capabilities: Vec<Capability>) -> Self {
        CapNegotiator {
            requested: capabilities,
            acknowledged: vec![],
            not_acknowledged: vec![],
        }
    }

    pub fn ls(&self, version: Option<&str>) -> Message {
        let mut builder = Message::cmd("CAP").param("LS");

        if let Some(version) = version {
            builder = builder.param(version.as_ref());
        }

        builder.build()
    }

    pub fn end(&self) -> Message {
        Message::cmd("CAP").param("END").build()
    }

    pub fn handle(&mut self, message: Message) -> Result {
        if Command::Cmd("CAP".into()) != message.command {
            return Err(Error::UnexpectedCommand(message.command));
        }

        let nick = &message.parameters[0];
        let subcmd = &message.parameters[1];
        let param = &message.parameters[2..];

        println!("{nick:?} {subcmd:?} {param:?}");

        match subcmd.as_ref() {
            "LS" => self.match_listed_capabilities(param.to_vec()),
            "ACK" => self.ack(param.to_vec()),
            "NAK" => self.nak(param.to_vec()),
            _ => Err(Error::UnexpectedSubcommand(subcmd.to_string())),
        }
    }

    fn match_listed_capabilities(&self, params: Vec<AttrValue>) -> Result {
        let mut request: Vec<AttrValue> = vec![];

        // check if input parameters contain any requested capabilities
        for param in params {
            for capability in param.split(" ") {
                let capability = Capability(capability.to_string().into());
                if self.requested.contains(&capability) {
                    request.push(capability.0);
                }
            }
        }

        // do not send unnecessary empty requests
        if request.is_empty() {
            Ok(vec![])
        } else {
            Ok(vec![Message::cmd("CAP")
                .param("REQ")
                .param(&request.join(" "))
                .build()])
        }
    }

    fn nak(&mut self, params: Vec<AttrValue>) -> Result {
        for param in params {
            for cap in param.split(" ") {
                // insert capability into list of acknowledged capabilities
                let cap = Capability(cap.to_string().into());
                self.not_acknowledged.push(cap.clone());

                // remove from list of requested capabilities
                if let Some(requested_idx) = self.requested.iter().position(|c| c == &cap) {
                    self.requested.swap_remove(requested_idx);
                }
            }
        }
        Ok(vec![])
    }

    fn ack(&mut self, params: Vec<AttrValue>) -> Result {
        for param in params {
            for cap in param.split(" ") {
                // insert capability into list of acknowledged capabilities
                let cap = Capability(cap.to_string().into());
                self.acknowledged.push(cap.clone());

                // remove from list of requested capabilities
                if let Some(requested_idx) = self.requested.iter().position(|c| c == &cap) {
                    self.requested.swap_remove(requested_idx);
                }
            }
        }
        Ok(vec![])
    }
}
