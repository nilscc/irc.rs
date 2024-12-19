use pest_derive::Parser;

#[cfg(test)]
mod test;

#[derive(Parser)]
#[grammar = "irc.pest"]
pub struct Grammar;
