use std::{collections::BTreeMap, fmt::Display};

use generic_message::GenericMessage;
use message::{Message, MessageType};
use msg_cap::MsgCap;
use pest::{
    error::{Error, ErrorVariant},
    iterators::{Pair, Pairs},
    Parser, Position,
};

mod grammar;
use grammar::{Grammar, Rule};
use yew::AttrValue;

pub mod generic_message;
pub mod message;
pub mod msg_cap;

pub mod capability;

#[cfg(test)]
mod test;

#[derive(Debug)]
pub enum MessageBuilderError {
    MultipleTrailingParameters,
}

#[derive(Debug, PartialEq, Clone)]
pub enum Source {
    Host(AttrValue),
    User(User),
}

#[derive(Debug, PartialEq, Clone)]
pub struct User {
    pub nick: AttrValue,
    pub user: Option<AttrValue>,
    pub host: Option<AttrValue>,
}

#[derive(Debug, PartialEq, Clone)]
pub enum Command {
    Digit3(u32),
    Cmd(AttrValue),
}

impl Command {
    pub fn valid(&self) -> bool {
        match self {
            Self::Digit3(val) => 1 <= *val && *val <= 999,
            Self::Cmd(cmd) => !cmd.is_empty(),
        }
    }
}

impl Display for Command {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Digit3(val) => write!(f, "{val:0>3}"),
            Self::Cmd(cmd) => write!(f, "{cmd}"),
        }
    }
}

fn unexpected_rule(pair: Pair<Rule>) -> Error<Rule> {
    Error::new_from_span(
        ErrorVariant::CustomError {
            message: format!("Unexpected rule: {:?}", pair.as_rule()),
        },
        pair.as_span(),
    )
}

fn empty_pairs(pairs: &Pairs<Rule>) -> Error<Rule> {
    Error::new_from_pos(
        ErrorVariant::CustomError {
            message: "Empty pairs.".into(),
        },
        Position::new(pairs.get_input(), 0).unwrap(),
    )
}

fn too_many_pairs(pairs: &Pairs<Rule>, expected: u32) -> Error<Rule> {
    Error::new_from_pos(
        ErrorVariant::CustomError {
            message: format!("Expected {} pairs, have: {}", expected, pairs.len()),
        },
        Position::new(pairs.get_input(), 0).unwrap(),
    )
}

impl Message {
    pub fn parse(str: &str) -> Result<Self, Error<Rule>> {
        let mut pairs = Grammar::parse(Rule::message, str)?;
        match pairs.next() {
            Some(pair) if pair.as_rule() == Rule::message => Self::parse_inner(pair.into_inner()),
            _ => Err(Error::new_from_pos(
                pest::error::ErrorVariant::CustomError {
                    message: "Failed.".into(),
                },
                Position::new(str, 0).unwrap(),
            )),
        }
    }

    fn parse_inner(pairs: Pairs<Rule>) -> Result<Self, Error<Rule>> {
        let mut tags = BTreeMap::new();
        let mut source = None;
        let mut msg_type = None::<MessageType>;

        // parse inner pairs
        for pair in pairs {
            match pair.as_rule() {
                Rule::tags => tags = Self::parse_tags(pair.into_inner())?,
                Rule::source => source = Some(Self::parse_source(pair.into_inner())?),
                Rule::msg_type => msg_type = Some(Self::parse_msg_type(pair.into_inner())?),
                _ => return Err(unexpected_rule(pair)),
            }
        }

        Ok(Message {
            tags,
            source,
            msg_type: msg_type.expect("Missing message type"),
        })
    }

    fn parse_msg_type(mut pairs: Pairs<Rule>) -> Result<MessageType, Error<Rule>> {
        if pairs.len() != 1 {
            return Err(too_many_pairs(&pairs, 1));
        }

        let pair = pairs.next().unwrap();
        match pair.as_rule() {
            Rule::generic_message => Ok(MessageType::Generic(GenericMessage::parse(
                pair.into_inner(),
            )?)),
            Rule::msg_cap => Ok(MessageType::Capability(MsgCap::parse(pair.into_inner())?)),
            _ => return Err(unexpected_rule(pair)),
        }
    }

    fn parse_tags(pairs: Pairs<Rule>) -> Result<BTreeMap<String, Option<String>>, Error<Rule>> {
        let mut tags = BTreeMap::new();
        for pair in pairs {
            match pair.as_rule() {
                Rule::tag => {
                    let mut key = "";
                    let mut value = None::<String>;

                    for pair in pair.into_inner() {
                        match pair.as_rule() {
                            Rule::key => key = pair.as_str(),
                            Rule::assignment => value = Some(String::new()),
                            Rule::escaped_value => value = Some(pair.as_str().into()),
                            _ => return Err(unexpected_rule(pair.clone())),
                        }
                    }
                    tags.insert(key.to_owned(), value);
                }
                _ => return Err(unexpected_rule(pair.clone())),
            }
        }
        Ok(tags)
    }

    fn parse_source(mut pairs: Pairs<Rule>) -> Result<Source, Error<Rule>> {
        let pair = pairs.next().ok_or(empty_pairs(&pairs))?;
        print!("{pair:?}");

        if pair.as_rule() != Rule::name {
            return Err(unexpected_rule(pair));
        }
        let name = pair.as_str().to_owned().into();

        // lookup user and host (if they exist)
        let mut user = None::<AttrValue>;
        let mut host = None::<AttrValue>;
        for pair in pairs {
            match pair.as_rule() {
                Rule::user => user = Some(pair.as_str().to_owned().into()),
                Rule::host => host = Some(pair.as_str().to_owned().into()),
                _ => return Err(unexpected_rule(pair)),
            }
        }

        Ok(if user.is_none() && host.is_none() {
            Source::Host(name)
        } else {
            Source::User(User {
                nick: name,
                user,
                host,
            })
        })
    }
}
