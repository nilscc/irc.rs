use std::fmt::Display;

use yew::AttrValue;

#[derive(Debug, Clone, PartialEq)]
pub enum Capability {
    Single(AttrValue),
    Values(AttrValue, Vec<AttrValue>),
    Disabled(AttrValue),
}

impl Capability {
    pub fn new(value: &str) -> Self {
        Capability::Single(value.to_owned().into())
    }

    pub fn values(key: &str, values: Vec<&'static str>) -> Self {
        Capability::Values(
            key.to_owned().into(),
            values.iter().map(|s| s.to_owned().into()).collect(),
        )
    }

    pub fn disabled(value: &str) -> Self {
        Capability::Disabled(value.to_owned().into())
    }
}

impl Display for Capability {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Capability::Single(val) => write!(f, "{val}"),
            Capability::Disabled(val) => write!(f, "-{val}"),
            Capability::Values(key, values) => write!(f, "{key}={}", values.join(",")),
        }
    }
}
