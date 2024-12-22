use core::panic;
use std::collections::BTreeMap;

use super::{Command, Message, Source, User};

fn parse(input: &str) -> Message {
    match Message::parse(input) {
        Err(err) => panic!("{err}"),
        Ok(msg) => msg,
    }
}

/// Empty input should not deliver valid message
#[test]
#[should_panic]
fn test_message_parse_empty_input() {
    parse("");
}

/// Simple command with no parameters
#[test]
fn test_message_parse_simple_command() {
    let msg = parse("PING");
    assert_eq!(
        msg,
        Message {
            tags: BTreeMap::new(),
            source: None,
            command: Command::Cmd("PING".into()),
            parameters: vec![],
        }
    )
}

/// Simple 3digit command with no parameters
#[test]
fn test_message_parse_3digit_command() {
    let msg = parse("001");
    assert_eq!(
        msg,
        Message {
            tags: BTreeMap::new(),
            source: None,
            command: Command::Digit3(1),
            parameters: vec![],
        }
    );

    // check string generation
    assert_eq!("001", msg.command.to_string());
}

/// Simple command with single parameters
#[test]
fn test_message_parse_single_param() {
    let msg = parse("NICK nick");
    assert_eq!(
        msg,
        Message {
            tags: BTreeMap::new(),
            source: None,
            command: Command::Cmd("NICK".into()),
            parameters: vec!["nick".into()],
        }
    )
}

/// Simple command with empty trailing parameter
#[test]
fn test_message_parse_empty_trailing_param() {
    let msg = parse("TEST :");
    assert_eq!(
        msg,
        Message {
            tags: BTreeMap::new(),
            source: None,
            command: Command::Cmd("TEST".into()),
            parameters: vec!["".into()],
        }
    )
}

/// Command with multiple parameters
#[test]
fn test_message_parse_multiple_param() {
    let msg = parse("CAP * LS :draft/example-1 draft/example-2");
    assert_eq!(
        msg,
        Message {
            tags: BTreeMap::new(),
            source: None,
            command: Command::Cmd("CAP".into()),
            parameters: vec![
                "*".into(),
                "LS".into(),
                "draft/example-1 draft/example-2".into(),
            ],
        }
    )
}

#[test]
fn test_command_with_tags_id_rose() {
    let msg = parse("@id=123AB;rose TEST");
    assert_eq!(
        msg,
        Message {
            tags: BTreeMap::from([("id".into(), Some("123AB".into())), ("rose".into(), None)]),
            source: None,
            command: Command::Cmd("TEST".into()),
            parameters: vec![],
        }
    );
}

#[test]
fn test_command_with_tags_url_netsplit() {
    let msg = parse("@url=;netsplit=tur,ty TEST");
    assert_eq!(
        msg,
        Message {
            tags: BTreeMap::from([
                ("netsplit".into(), Some("tur,ty".into())),
                ("url".into(), None),
            ]),
            source: None,
            command: Command::Cmd("TEST".into()),
            parameters: vec![],
        }
    );
}

#[test]
fn test_source_host() {
    let msg = parse(":irc.example.com TEST");
    assert_eq!(
        msg,
        Message {
            tags: BTreeMap::new(),
            source: Some(Source::Host("irc.example.com".into())),
            command: Command::Cmd("TEST".into()),
            parameters: vec![],
        }
    );
}

#[test]
fn test_source_user() {
    let msg = parse(":dan!d@localhost TEST");
    assert_eq!(
        msg,
        Message {
            tags: BTreeMap::new(),
            source: Some(Source::User(User {
                nick: "dan".into(),
                user: Some("d".into()),
                host: Some("localhost".into()),
            })),
            command: Command::Cmd("TEST".into()),
            parameters: vec![],
        }
    );
}
