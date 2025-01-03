use std::{collections::BTreeMap, fmt::Display};

use yew::AttrValue;

use crate::irc::parser::User;

use super::{generic_message::GenericMessage, msg_cap::MsgCap, Source};

#[derive(Debug, PartialEq, Clone)]
pub struct Message {
    pub tags: BTreeMap<String, Option<String>>,
    pub source: Option<Source>,
    pub msg_type: MessageType,
}

impl Message {
    pub fn builder() -> MessageBuilder {
        MessageBuilder::new()
    }

    pub fn generic(msg: GenericMessage) -> MessageBuilder {
        let msg_type = MessageType::Generic(msg);
        MessageBuilder::new().msg_type(msg_type)
    }

    pub fn cmd(command: &str) -> MessageBuilder {
        Self::generic(GenericMessage::cmd(command))
    }

    pub fn digit3(digit: u32) -> MessageBuilder {
        Self::generic(GenericMessage::digit3(digit))
    }

    pub fn cap(msg_cap: MsgCap) -> MessageBuilder {
        let msg_type = MessageType::Capability(msg_cap);
        MessageBuilder::new().msg_type(msg_type)
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum MessageType {
    Generic(GenericMessage),
    Capability(MsgCap),
}

impl Display for MessageType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            MessageType::Generic(msg) => write!(f, "{msg}"),
            MessageType::Capability(cap) => write!(f, "{cap}"),
        }
    }
}

impl Display for Message {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // format list of tags
        if !self.tags.is_empty() {
            write!(
                f,
                "@{} ",
                self.tags
                    .iter()
                    .map(|(key, mval)| format!(
                        "{key}{}",
                        mval.clone().map_or(String::new(), |val| format!("={val}"))
                    ))
                    .collect::<Vec<String>>()
                    .join(";")
            )?;
        }

        // format source
        if let Some(src) = &self.source {
            write!(
                f,
                ":{} ",
                match src {
                    Source::Host(name) => name.to_string(),
                    Source::User(User { nick, user, host }) => format!(
                        "{nick}{}{}",
                        user.clone()
                            .map_or(String::new(), |user| format!("!{user}")),
                        host.clone()
                            .map_or(String::new(), |host| format!("@{host}")),
                    ),
                }
            )?;
        }

        write!(f, "{}", self.msg_type)
    }
}

pub struct MessageBuilder {
    tags: BTreeMap<String, Option<String>>,
    source: Option<Source>,
    msg_type: Option<MessageType>,
}

impl MessageBuilder {
    fn new() -> MessageBuilder {
        MessageBuilder {
            tags: BTreeMap::new(),
            source: None,
            msg_type: None,
        }
    }

    /// Panics if msg_type is `None`!
    pub fn build(self) -> Message {
        Message {
            tags: self.tags,
            source: self.source,
            msg_type: self.msg_type.unwrap(),
        }
    }

    pub fn msg_type(mut self, msg_type: MessageType) -> Self {
        self.msg_type = Some(msg_type);
        self
    }

    pub fn param(mut self, parameter: &str) -> Self {
        match &mut self.msg_type {
            Some(MessageType::Generic(msg)) => msg.parameters.push(parameter.to_owned().into()),
            _ => panic!("Builder does not support non-generic messages."),
        }
        self
    }

    pub fn parameters(mut self, parameters: Vec<AttrValue>) -> Self {
        match &mut self.msg_type {
            Some(MessageType::Generic(msg)) => msg.parameters = parameters,
            _ => panic!("Builder does not support non-generic messages."),
        }
        self
    }

    pub fn tag(mut self, key: &str, value: Option<&str>) -> Self {
        self.tags
            .insert(key.to_owned(), value.map(|str| str.to_owned()));
        self
    }

    pub fn host(mut self, host: &str) -> Self {
        self.source = Some(Source::Host(host.to_owned().into()));
        self
    }

    pub fn user(mut self, user: User) -> Self {
        self.source = Some(Source::User(user));
        self
    }
}
