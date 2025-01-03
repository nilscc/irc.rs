use crate::irc::{client::Capability, parser::msg_cap::MsgCap};

use super::CapNegotiator;

#[test]
fn test_empty_request() {
    let mut negotiator = CapNegotiator::request(vec![]);
    assert_eq!(negotiator.requested.len(), 0);

    let ls = negotiator.ls(Some("302".into()));

    assert_eq!(ls.to_string(), "CAP LS 302");

    let msgs = negotiator
        .handle(
            MsgCap::builder()
                .star()
                .ls(false)
                .single("multi-prefix")
                .single("sasl")
                .build(),
        )
        .unwrap();

    println!("{msgs:?}");

    // there should be no response messages
    assert_eq!(msgs.len(), 0)
}

#[test]
#[ignore = "need to be correctly implemented again"]
fn test_sinlge_request() {
    let mut negotiator = CapNegotiator::request(vec![Capability::new("sasl")]);
    assert_eq!(negotiator.requested.len(), 1);

    let ls = negotiator.ls(Some("302".into()));

    assert_eq!(ls.to_string(), "CAP LS 302");

    let msgs = negotiator
        .handle(
            MsgCap::builder()
                .star()
                .ls(false)
                .single("multi-prefix")
                .single("sasl")
                .build(),
        )
        .unwrap();

    println!("{msgs:?}");

    // response message should contain requested capability
    assert_eq!(msgs.len(), 1);
    assert_eq!(msgs[0].to_string(), "CAP REQ sasl");
}

#[test]
#[ignore = "need to be correctly implemented again"]
fn test_multiple_requests() {
    let mut negotiator = CapNegotiator::request(vec![
        Capability::new("sasl"),
        Capability::new("multi-prefix"),
    ]);
    assert_eq!(negotiator.requested.len(), 2);

    let ls = negotiator.ls(Some("302".into()));

    assert_eq!(ls.to_string(), "CAP LS 302");

    let msgs = negotiator
        .handle(
            MsgCap::builder()
                .star()
                .ls(false)
                .single("multi-prefix")
                .single("sasl")
                .build(),
        )
        .unwrap();

    println!("{msgs:?}");

    // response message should contain both capabilities
    assert_eq!(msgs.len(), 1);
    assert_eq!(msgs[0].to_string(), "CAP REQ :multi-prefix sasl");

    // test ack/nak
    let msgs = negotiator
        .handle(MsgCap::builder().star().ack().single("sasl").build())
        .unwrap();
    assert_eq!(msgs, vec![]);

    assert!(negotiator.acknowledged.contains(&Capability::new("sasl")));

    let msgs = negotiator
        .handle(
            MsgCap::builder()
                .star()
                .nak()
                .single("multi-prefix")
                .build(),
        )
        .unwrap();
    assert_eq!(msgs, vec![]);

    assert!(negotiator
        .not_acknowledged
        .contains(&Capability::new("multi-prefix")));

    assert!(negotiator.requested.is_empty());

    let msgs = negotiator.end();

    assert_eq!(msgs.to_string(), "CAP END");
}
