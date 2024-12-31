use core::panic;

use pest::Parser;

use crate::irc::parser::{
    capability::Capability,
    grammar::{Grammar, Rule},
    msg_cap::{CapNick, SubCommand},
};

use super::MsgCap;

fn parse(input: &str) -> MsgCap {
    let res = Grammar::parse(Rule::msg_cap, input);
    match res {
        Err(err) => panic!("{err}"),
        Ok(mut pairs) => MsgCap::parse(pairs.next().unwrap().into_inner()).unwrap(),
    }
}

#[test]
fn test_star() {
    let input = "CAP * LS :";
    let msg = parse(input);

    assert_eq!(msg.nick, CapNick::Star);
    assert_eq!(msg.sub_command, SubCommand::LS(false, vec![]));
}

#[test]
fn test_nick() {
    let input = "CAP customclient LS :";
    let msg = parse(input);

    assert_eq!(msg.nick, CapNick::nick("customclient"));
    assert_eq!(msg.sub_command, SubCommand::LS(false, vec![]));
}

#[test]
fn test_ls() {
    let input = "CAP * LS * :one two=three,four -five";
    let msg = parse(input);

    assert_eq!(msg.nick, CapNick::Star);
    assert_eq!(
        msg.sub_command,
        SubCommand::LS(
            true,
            vec![
                Capability::new("one"),
                Capability::values("two", vec!["three", "four"]),
                Capability::disabled("five"),
            ]
        )
    );
}
