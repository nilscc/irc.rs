use pest_derive::Parser;

#[cfg(test)]
mod test;

#[derive(Parser)]
#[grammar = "irc/parser/capability.pest"]
struct Grammar {}
