use std::{collections::BTreeMap, fmt::Display};

use pest::{
    error::{Error, ErrorVariant},
    iterators::{Pair, Pairs},
    Parser, Position,
};

mod grammar;
use grammar::{Grammar, Rule};

#[cfg(test)]
mod test;

#[derive(Debug, PartialEq, Clone)]
pub struct Message {
    pub tags: BTreeMap<String, Option<String>>,
    pub source: Option<Source>,
    pub command: Command,
    pub parameters: Vec<String>,
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
                    Source::Host(name) => name.clone(),
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

        // format command
        match &self.command {
            Command::Cmd(cmd) => write!(f, "{cmd}")?,
            Command::Digit3(digit3) => write!(f, "{digit3:0>3}")?,
        }

        // format parameters
        match &self.parameters[..] {
            [middle @ .., trailing] => {
                if !middle.is_empty() {
                    write!(f, " {}", middle.join(" "))?;
                }
                if !trailing.is_empty() {
                    let prefix = if trailing.contains(" ") { ":" } else { "" };
                    write!(f, " {prefix}{trailing}")?;
                }
            }
            _ => {}
        }

        //let len = self.parameters.len();
        //write!(f, " {}", self.parameters[..len - 1].join(" "))?;
        //self.parameters.last().map_or(Ok(()), |s| {
        //    let trailing =
        //    write!(f, " {trailing}{s}")
        //})?;

        Ok(())
    }
}

#[derive(Debug)]
pub enum MessageBuilderError {
    MultipleTrailingParameters,
}

pub struct MessageBuilder {
    message: Message,
}

impl Message {
    pub fn new(command: Command) -> Self {
        Message {
            command,
            tags: BTreeMap::new(),
            source: None,
            parameters: vec![],
        }
    }

    pub fn builder(command: Command) -> MessageBuilder {
        MessageBuilder {
            message: Self::new(command),
        }
    }

    pub fn cmd(command: &str) -> MessageBuilder {
        Self::builder(Command::Cmd(command.into()))
    }
}

impl MessageBuilder {
    pub fn build(self) -> Message {
        self.message
    }

    pub fn param(mut self, parameter: &str) -> MessageBuilder {
        self.message.parameters.push(parameter.into());
        self
    }

    pub fn parameters(mut self, parameters: Vec<String>) -> MessageBuilder {
        self.message.parameters = parameters;
        self
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum Source {
    Host(String),
    User(User),
}

#[derive(Debug, PartialEq, Clone)]
pub struct User {
    nick: String,
    user: Option<String>,
    host: Option<String>,
}

#[derive(Debug, PartialEq, Clone)]
pub enum Command {
    Digit3(u32),
    Cmd(String),
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

    fn parse_inner(pairs: Pairs<Rule>) -> Result<Self, Error<Rule>> {
        let mut tags = BTreeMap::new();
        let mut source = None;
        let mut parameters = vec![];
        let mut command = Command::Digit3(0);

        // parse inner pairs
        for pair in pairs {
            match pair.as_rule() {
                Rule::tags => tags = Self::parse_tags(pair.into_inner())?,
                Rule::source => source = Some(Self::parse_source(pair.into_inner())?),
                Rule::command => command = Self::parse_command(pair)?,
                Rule::parameters => {
                    parameters.append(&mut Self::parse_parameters(pair.into_inner())?)
                }
                _ => return Err(Self::unexpected_rule(pair)),
            }
        }

        Ok(Message {
            tags,
            source,
            parameters,
            command,
        })
    }

    fn parse_tags(pairs: Pairs<Rule>) -> Result<BTreeMap<String, Option<String>>, Error<Rule>> {
        let mut tags = BTreeMap::new();
        for pair in pairs {
            match pair.as_rule() {
                Rule::tag => {
                    let mut inner = pair.into_inner();
                    let key = match inner.next() {
                        Some(k) if k.as_rule() == Rule::key => k.as_str(),
                        _ => todo!(),
                    };
                    let val = match inner.next() {
                        // TODO: unescape
                        Some(v) if v.as_rule() == Rule::escaped_value => {
                            Some(v.as_str().to_owned())
                        }
                        _ => None,
                    };
                    tags.insert(key.to_owned(), val);
                }
                _ => return Err(Self::unexpected_rule(pair.clone())),
            }
        }
        Ok(tags)
    }

    fn parse_source(mut pairs: Pairs<Rule>) -> Result<Source, Error<Rule>> {
        let pair = pairs.next().ok_or(Self::empty_pairs(&pairs))?;
        print!("{pair:?}");

        if pair.as_rule() != Rule::name {
            return Err(Self::unexpected_rule(pair));
        }
        let name = pair.as_str().to_owned();

        // lookup user and host (if they exist)
        let mut user = None::<String>;
        let mut host = None::<String>;
        for pair in pairs {
            match pair.as_rule() {
                Rule::user => user = Some(pair.as_str().to_owned()),
                Rule::host => host = Some(pair.as_str().to_owned()),
                _ => return Err(Self::unexpected_rule(pair)),
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

    fn parse_command(pair: Pair<Rule>) -> Result<Command, Error<Rule>> {
        let cmd = pair.as_str().to_owned();
        match pair.into_inner().next() {
            Some(val) if val.as_rule() == Rule::digit3 => Ok(Command::Digit3(cmd.parse().unwrap())),
            _ => Ok(Command::Cmd(cmd)),
        }
    }

    fn parse_parameters(pairs: Pairs<Rule>) -> Result<Vec<String>, Error<Rule>> {
        let mut params = Vec::<String>::new();
        for pair in pairs {
            match pair.as_rule() {
                Rule::middle => params.push(pair.as_str().to_owned()),
                Rule::trailing => {
                    params.push(pair.into_inner().next().unwrap().as_str().to_owned())
                }
                _ => return Err(Self::unexpected_rule(pair)),
            }
        }
        Ok(params)
    }
}
