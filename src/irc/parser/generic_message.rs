use std::fmt::Display;

use pest::{
    error::Error,
    iterators::{Pair, Pairs},
};
use yew::AttrValue;

use super::{grammar::Rule, unexpected_rule, Command};

#[cfg(test)]
mod test;

#[derive(Debug, PartialEq, Clone)]
pub struct GenericMessage {
    pub command: Command,
    pub parameters: Vec<AttrValue>,
}

/// Construction impls
impl GenericMessage {
    pub fn new(command: Command) -> Self {
        GenericMessage {
            command,
            parameters: vec![],
        }
    }

    pub fn cmd(cmd: &str) -> Self {
        Self::new(Command::Cmd(cmd.to_string().into()))
    }

    pub fn digit3(digit: u32) -> Self {
        Self::new(Command::Digit3(digit))
    }

    pub fn param(mut self, param: &str) -> Self {
        self.parameters.push(param.to_string().into());
        self
    }
}

impl GenericMessage {
    pub fn parse(pairs: Pairs<Rule>) -> Result<Self, Error<Rule>> {
        let mut command = None::<Command>;
        let mut parameters = vec![];

        for pair in pairs {
            match pair.as_rule() {
                Rule::command => command = Some(Self::parse_command(pair)?),
                Rule::parameters => parameters = Self::parse_parameters(pair.into_inner())?,
                r => todo!("Unexpected rule {r:?}"),
            }
        }

        match command {
            Some(command) => Ok(GenericMessage {
                command,
                parameters,
            }),
            _ => todo!("Missing command"),
        }
    }

    fn parse_command(pair: Pair<Rule>) -> Result<Command, Error<Rule>> {
        let cmd = pair.as_str().to_owned();
        match pair.into_inner().next() {
            Some(val) if val.as_rule() == Rule::digit3 => Ok(Command::Digit3(cmd.parse().unwrap())),
            _ => Ok(Command::Cmd(cmd.into())),
        }
    }

    fn parse_parameters(pairs: Pairs<Rule>) -> Result<Vec<AttrValue>, Error<Rule>> {
        let mut params = Vec::<AttrValue>::new();
        for pair in pairs {
            match pair.as_rule() {
                Rule::middle => params.push(pair.as_str().to_owned().into()),
                Rule::trailing => {
                    params.push(pair.into_inner().next().unwrap().as_str().to_owned().into())
                }
                _ => return Err(unexpected_rule(pair)),
            }
        }
        Ok(params)
    }
}

impl Display for GenericMessage {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
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

        Ok(())
    }
}
