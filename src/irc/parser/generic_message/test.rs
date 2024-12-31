use pest::Parser;

use crate::irc::parser::{
    grammar::{Grammar, Rule},
    Command,
};

use super::GenericMessage;

#[test]
fn test_generic_message() {
    let input = "TEST a b c :x y z";

    let mut pairs = Grammar::parse(Rule::generic_message, input).unwrap();
    let pair = pairs.next().unwrap();
    let msg = GenericMessage::parse(pair.into_inner()).unwrap();
    assert_eq!(
        msg,
        GenericMessage {
            command: Command::Cmd("TEST".to_string()),
            parameters: vec!["a", "b", "c", "x y z"]
                .iter()
                .map(|s| s.to_owned().into())
                .collect(),
        }
    );
}
