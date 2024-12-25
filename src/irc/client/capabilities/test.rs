use crate::irc::parser::Message;

use super::CapNegotiator;

#[test]
fn test_empty_request() {
    let mut negotiator = CapNegotiator::request(vec![]);

    let ls = negotiator.ls(Some("302".into()));

    assert_eq!(ls.to_string(), "CAP LS 302");

    let msgs = negotiator.handle(
        Message::cmd("CAP")
            .param("*")
            .param("LS")
            .param("multi-prefix sasl")
            .build()
            .unwrap(),
    );

    // there should be no response messages
    assert_eq!(msgs.unwrap().len(), 0)
}
