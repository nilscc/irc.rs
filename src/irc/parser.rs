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

#[derive(Debug, PartialEq, Clone)]
pub struct Source {}

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

    fn parse_source(_pairs: Pairs<Rule>) -> Result<Source, Error<Rule>> {
        todo!()
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
