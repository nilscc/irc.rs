use std::fmt::{Display, Formatter};

use pest::{error::Error, iterators::Pairs};
use yew::AttrValue;

use super::{capability::Capability, grammar::Rule, unexpected_rule};

#[cfg(test)]
mod test;

#[derive(Debug, PartialEq, Clone)]
pub struct MsgCap {
    pub nick: CapNick,
    pub sub_command: SubCommand,
}

#[derive(Debug, PartialEq, Clone)]
pub enum CapNick {
    Star,
    Nick(AttrValue),
}

impl CapNick {
    pub fn nick(nick: &str) -> Self {
        CapNick::Nick(nick.to_owned().into())
    }
}

pub type Multiline = bool;

pub type Capabilities = Vec<Capability>;

#[derive(Debug, PartialEq, Clone)]
pub enum SubCommand {
    LS(Multiline, Capabilities),
    LIST(Multiline, Capabilities),
    REQ(Capabilities),
    ACK(Capabilities),
    NAK(Capabilities),
    NEW(Capabilities),
    DEL(Capabilities),
}

impl SubCommand {
    fn join_capabilities(capabilities: &Capabilities) -> String {
        capabilities
            .iter()
            .map(|s| s.to_string())
            .collect::<Vec<String>>()
            .join(" ")
    }

    fn command_name(&self) -> &'static str {
        use SubCommand::*;
        match self {
            LS(_, _) => "LS",
            LIST(_, _) => "LIST",
            REQ(_) => "REQ",
            ACK(_) => "ACK",
            NAK(_) => "NAK",
            NEW(_) => "NEW",
            DEL(_) => "DEL",
        }
    }
}

impl Display for SubCommand {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        use SubCommand::*;
        match self {
            LS(multi, caps) | LIST(multi, caps) => write!(
                f,
                "{} {}:{}",
                self.command_name(),
                if *multi { "* " } else { "" },
                Self::join_capabilities(caps),
            ),
            REQ(c) | ACK(c) | NAK(c) | NEW(c) | DEL(c) => {
                write!(f, "{} :{}", self.command_name(), Self::join_capabilities(c))
            }
        }
    }
}

/// Parsing `MsgCap` implementation
impl MsgCap {
    pub fn parse(pairs: Pairs<Rule>) -> Result<Self, Error<Rule>> {
        let mut nick = None::<CapNick>;
        let mut sub_command = None::<SubCommand>;

        for pair in pairs {
            match pair.as_rule() {
                Rule::cap_nick => nick = Some(Self::parse_nick(pair.into_inner())?),
                Rule::cap_cmd => sub_command = Some(Self::parse_sub_command(pair.into_inner())?),
                _ => todo!("{pair:?}"),
            }
        }

        Ok(MsgCap {
            nick: nick.unwrap(), // TODO: proper error handling
            sub_command: sub_command.unwrap(),
        })
    }

    fn parse_nick(pairs: Pairs<Rule>) -> Result<CapNick, Error<Rule>> {
        // use peek() instead of next() to not actually consume the first character
        let pair = pairs.peek().expect("At least one pair");
        match pair.as_rule() {
            Rule::star => Ok(CapNick::Star),
            Rule::nospcrlf => Ok(CapNick::nick(pairs.as_str())),
            _ => Err(unexpected_rule(pair)),
        }
    }

    fn parse_sub_command(mut pairs: Pairs<Rule>) -> Result<SubCommand, Error<Rule>> {
        let pair = pairs.next().unwrap();
        match pair.as_rule() {
            Rule::cap_ls => Ok(Self::parse_ls(pair.into_inner())?),
            _ => todo!("{pair:?}"),
        }
    }

    fn parse_ls(pairs: Pairs<Rule>) -> Result<SubCommand, Error<Rule>> {
        let mut multiline = false;
        let mut capabilities = vec![];

        for pair in pairs {
            match pair.as_rule() {
                Rule::multiline => multiline = true,
                Rule::capability => capabilities.push(Self::parse_capability(pair.into_inner())?),
                _ => return Err(unexpected_rule(pair)),
            }
        }
        Ok(SubCommand::LS(multiline, capabilities))
    }

    fn parse_capability(pairs: Pairs<Rule>) -> Result<Capability, Error<Rule>> {
        let mut disabled = false;
        let mut key = "";
        let mut values = vec![];

        for pair in pairs {
            match pair.as_rule() {
                Rule::minus => disabled = true,
                Rule::cap_key => key = pair.as_str(),
                Rule::assignment => (),
                Rule::cap_values => {
                    for pair in pair.into_inner() {
                        match pair.as_rule() {
                            Rule::cap_value => values.push(pair.as_str().to_owned().into()),
                            _ => return Err(unexpected_rule(pair)),
                        }
                    }
                }
                _ => return Err(unexpected_rule(pair)),
            }
        }

        Ok(if disabled {
            Capability::disabled(key)
        } else if values.is_empty() {
            Capability::new(key)
        } else {
            Capability::Values(key.to_owned().into(), values)
        })
    }
}

impl Display for MsgCap {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let nick = match &self.nick {
            CapNick::Star => "*",
            CapNick::Nick(nick) => nick,
        };

        write!(f, "CAP {nick} {sub_command}")
    }
}
