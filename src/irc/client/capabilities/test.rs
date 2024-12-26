use crate::irc::{client::Capability, parser::Message};

use super::CapNegotiator;

#[test]
fn test_empty_request() {
    let mut negotiator = CapNegotiator::request(vec![]);
    assert_eq!(negotiator.requested.len(), 0);

    let ls = negotiator.ls(Some("302".into()));

    assert_eq!(ls.to_string(), "CAP LS 302");

    let msgs = negotiator
        .handle(
            Message::cmd("CAP")
                .param("*")
                .param("LS")
                .param("multi-prefix sasl")
                .build(),
        )
        .unwrap();

    println!("{msgs:?}");

    // there should be no response messages
    assert_eq!(msgs.len(), 0)
}

#[test]
fn test_sinlge_request() {
    let mut negotiator = CapNegotiator::request(vec![Capability("sasl".into())]);
    assert_eq!(negotiator.requested.len(), 1);

    let ls = negotiator.ls(Some("302".into()));

    assert_eq!(ls.to_string(), "CAP LS 302");

    let msgs = negotiator
        .handle(
            Message::cmd("CAP")
                .param("*")
                .param("LS")
                .param("multi-prefix sasl")
                .build(),
        )
        .unwrap();

    println!("{msgs:?}");

    // response message should contain requested capability
    assert_eq!(msgs.len(), 1);
    assert_eq!(msgs[0].to_string(), "CAP REQ sasl");
}

#[test]
fn test_multiple_requests() {
    let mut negotiator = CapNegotiator::request(vec![
        Capability("sasl".into()),
        Capability("multi-prefix".into()),
    ]);
    assert_eq!(negotiator.requested.len(), 2);

    let ls = negotiator.ls(Some("302".into()));

    assert_eq!(ls.to_string(), "CAP LS 302");

    let msgs = negotiator
        .handle(
            Message::cmd("CAP")
                .param("*")
                .param("LS")
                .param("multi-prefix sasl")
                .build(),
        )
        .unwrap();

    println!("{msgs:?}");

    // response message should contain both capabilities
    assert_eq!(msgs.len(), 1);
    assert_eq!(msgs[0].to_string(), "CAP REQ :multi-prefix sasl");

    // test ack/nak
    let msgs = negotiator
        .handle(
            Message::cmd("CAP")
                .param("*")
                .param("ACK")
                .param("sasl")
                .build(),
        )
        .unwrap();
    assert_eq!(msgs, vec![]);

    assert!(negotiator
        .acknowledged
        .contains(&Capability("sasl".to_string())));

    let msgs = negotiator
        .handle(
            Message::cmd("CAP")
                .param("*")
                .param("NAK")
                .param("multi-prefix")
                .build(),
        )
        .unwrap();
    assert_eq!(msgs, vec![]);

    assert!(negotiator
        .not_acknowledged
        .contains(&Capability("multi-prefix".to_string())));

    assert!(negotiator.requested.is_empty());

    let msgs = negotiator.end();

    assert_eq!(msgs.to_string(), "CAP END");
}
