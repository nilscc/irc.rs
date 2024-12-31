use core::panic;

use super::{Command, Message, User};

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
    let input = "PING";
    let msg = parse(input);
    assert_eq!(msg.to_string(), input);
    assert_eq!(msg, Message::cmd("PING").build())
}

/// Simple 3digit command with no parameters
#[test]
fn test_message_parse_3digit_command() {
    let input = "001";
    let msg = parse(input);
    assert_eq!(msg.to_string(), input);
    assert_eq!(msg, Message::builder(Command::Digit3(1)).build());

    // check string generation
    assert_eq!("001", msg.msg_type.to_string());
}

/// Simple command with single parameters
#[test]
fn test_message_parse_single_param() {
    let input = "NICK nick";
    let msg = parse(input);
    assert_eq!(msg.to_string(), input);
    assert_eq!(msg, Message::cmd("NICK").param("nick").build());
}

/// Simple command with empty trailing parameter
#[test]
fn test_message_parse_empty_trailing_param() {
    let msg = parse("TEST :");

    // empty trailing parameter does not need to be preserved
    assert_eq!(msg.to_string(), "TEST");

    assert_eq!(msg, Message::cmd("TEST").param("").build());
}

/// Command with multiple parameters
#[test]
fn test_message_parse_multiple_param() {
    let input = "TEST * :draft/example-1 draft/example-2";
    let msg = parse(input);
    assert_eq!(msg.to_string(), input);
    assert_eq!(
        msg,
        Message::cmd("TEST")
            .param("*")
            .param("draft/example-1 draft/example-2")
            .build()
    )
}

#[test]
fn test_command_with_tags_id_rose() {
    let input = "@id=123AB;rose TEST";
    let msg = parse(input);
    assert_eq!(msg.to_string(), input);
    assert_eq!(
        msg,
        Message::cmd("TEST")
            .tag("id", Some("123AB"))
            .tag("rose", None)
            .build()
    );
}

#[test]
fn test_command_with_tags_url_netsplit() {
    let msg = parse("@url=;netsplit=tur,ty TEST");
    assert_eq!(
        msg,
        Message::cmd("TEST")
            .tag("url", Some(""))
            .tag("netsplit", Some("tur,ty"))
            .build()
    );
}

#[test]
fn test_source_host() {
    let input = ":irc.example.com TEST";
    let msg = parse(input);
    assert_eq!(msg.to_string(), input);
    assert_eq!(msg, Message::cmd("TEST").host("irc.example.com").build());
}

#[test]
fn test_source_user() {
    let input = ":dan!d@localhost TEST";
    let msg = parse(input);
    assert_eq!(msg.to_string(), input);
    assert_eq!(
        msg,
        Message::cmd("TEST")
            .user(User {
                nick: "dan".into(),
                user: Some("d".into()),
                host: Some("localhost".into())
            })
            .build()
    );
}
