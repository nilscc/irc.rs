use yew::AttrValue;

/// IRCv3 Capability negotiation, following the spec:
///
/// https://ircv3.net/specs/extensions/capability-negotiation.html
use crate::irc::parser::{
    capability::Capability, message::Message, msg_cap::MsgCap, Command, MessageBuilderError,
};

#[cfg(test)]
mod test;

type Result = std::result::Result<Messages, Error>;
type Messages = Vec<Message>;

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

    pub fn handle(&mut self, message: MsgCap) -> Result {
        let nick = &message.nick;
        let subcmd = &message.sub_command;

        println!("{nick:?} {subcmd:?}");

        match subcmd {
            //"LS" => self.match_listed_capabilities(param.to_vec()),
            //"ACK" => self.ack(param.to_vec()),
            //"NAK" => self.nak(param.to_vec()),
            _ => Err(Error::UnexpectedSubcommand(subcmd.to_string())),
        }
    }

    // TODO:
    #[allow(dead_code)]
    fn match_listed_capabilities(&self, params: Vec<AttrValue>) -> Result {
        let mut request: Vec<AttrValue> = vec![];

        // check if input parameters contain any requested capabilities
        for param in params {
            for capability in param.split(" ") {
                let capability = Capability::new(capability);
                if self.requested.contains(&capability) {
                    request.push(capability.to_string().into());
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

    // TODO:
    #[allow(dead_code)]
    fn nak(&mut self, params: Vec<AttrValue>) -> Result {
        for param in params {
            for cap in param.split(" ") {
                // insert capability into list of acknowledged capabilities
                let cap = Capability::new(cap);
                self.not_acknowledged.push(cap.clone());

                // remove from list of requested capabilities
                if let Some(requested_idx) = self.requested.iter().position(|c| c == &cap) {
                    self.requested.swap_remove(requested_idx);
                }
            }
        }
        Ok(vec![])
    }

    // TODO:
    #[allow(dead_code)]
    fn ack(&mut self, params: Vec<AttrValue>) -> Result {
        for param in params {
            for cap in param.split(" ") {
                // insert capability into list of acknowledged capabilities
                let cap = Capability::new(cap);
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
