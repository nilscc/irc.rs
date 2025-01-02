use std::{collections::BTreeMap, fmt::Display};

use implicit_clone::unsync::IString;

use crate::irc::parser::User;

use super::{generic_message::GenericMessage, msg_cap::MsgCap, Command, Source};

#[derive(Debug, PartialEq, Clone)]
pub struct Message {
    pub tags: BTreeMap<String, Option<String>>,
    pub source: Option<Source>,
    pub msg_type: MessageType,
}

impl Message {
    pub fn new(msg_type: MessageType) -> Self {
        Message {
            tags: BTreeMap::new(),
            source: None,
            msg_type,
        }
    }

    pub fn generic(command: Command) -> MessageBuilder {
        let msg_type = MessageType::Generic(GenericMessage::new(command));
        MessageBuilder {
            message: Self::new(msg_type),
        }
    }

    pub fn cmd(command: &str) -> MessageBuilder {
        Self::generic(Command::Cmd(command.to_string().into()))
    }

    pub fn digit3(digit: u32) -> MessageBuilder {
        Self::generic(Command::Digit3(digit))
    }

    //pub fn cap() -> MessageBuilder {}
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
    message: Message,
}

impl MessageBuilder {
    pub fn build(self) -> Message {
        self.message
    }

    pub fn param(mut self, parameter: &str) -> Self {
        match &mut self.message.msg_type {
            MessageType::Generic(msg) => msg.parameters.push(parameter.to_owned().into()),
            _ => panic!("Builder does not support non-generic messages."),
        }
        self
    }

    pub fn parameters(mut self, parameters: Vec<IString>) -> Self {
        match &mut self.message.msg_type {
            MessageType::Generic(msg) => msg.parameters = parameters,
            _ => panic!("Builder does not support non-generic messages."),
        }
        self
    }

    pub fn tag(mut self, key: &str, value: Option<&str>) -> Self {
        self.message
            .tags
            .insert(key.to_owned(), value.map(|str| str.to_owned()));
        self
    }

    pub fn host(mut self, host: &str) -> Self {
        self.message.source = Some(Source::Host(host.to_owned().into()));
        self
    }

    pub fn user(mut self, user: User) -> Self {
        self.message.source = Some(Source::User(user));
        self
    }
}
