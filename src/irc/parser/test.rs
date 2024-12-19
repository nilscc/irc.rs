use std::collections::BTreeMap;

use crate::irc::parser::Command;

use super::Message;

/// Empty input should not deliver valid message
#[test]
fn test_message_parse_empty_input() {
    let res = Message::parse("");
    assert!(res.is_err());
}

/// Simple command with no parameters
#[test]
fn test_message_parse_simple_command() {
    let res = Message::parse("PING");
    assert_eq!(
        res,
        Ok(Message {
            tags: BTreeMap::new(),
            source: None,
            command: Command::Cmd("PING".into()),
            parameters: vec![],
        })
    )
}

/// Simple command with single parameters
#[test]
fn test_message_parse_single_param() {
    let res = Message::parse("NICK nick");
    assert_eq!(
        res,
        Ok(Message {
            tags: BTreeMap::new(),
            source: None,
            command: Command::Cmd("NICK".into()),
            parameters: vec!["nick".into()],
        })
    )
}

/// Simple command with empty trailing parameter
#[test]
fn test_message_parse_empty_trailing_param() {
    let res = Message::parse("TEST :");
    assert_eq!(
        res,
        Ok(Message {
            tags: BTreeMap::new(),
            source: None,
            command: Command::Cmd("TEST".into()),
            parameters: vec!["".into()],
        })
    )
}

/// Command with multiple parameters
#[test]
fn test_message_parse_multiple_param() {
    let res = Message::parse("CAP * LS :draft/example-1 draft/example-2");
    assert_eq!(
        res,
        Ok(Message {
            tags: BTreeMap::new(),
            source: None,
            command: Command::Cmd("CAP".into()),
            parameters: vec![
                "*".into(),
                "LS".into(),
                "draft/example-1 draft/example-2".into(),
            ],
        })
    )
}
