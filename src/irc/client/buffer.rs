use yew::AttrValue;

use crate::irc::parser::Source;

#[cfg(test)]
mod test;

#[derive(Debug, Clone, PartialEq)]
pub struct Buffer {
    pub id: u64,
    pub name: AttrValue,
    pub motd: Option<AttrValue>,
    pub lines: Vec<Line>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Line {
    pub source: Source,
    pub id: i64,
    pub message: AttrValue,
}
