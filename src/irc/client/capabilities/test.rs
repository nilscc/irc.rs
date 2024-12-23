use crate::irc::parser::Message;

use super::CapNegotiator;

#[test]
fn test_request() {
    let mut negotiator = CapNegotiator::request(vec![]);
    let msgs = negotiator.handle(
        Message::cmd("CAP")
            .param("*")
            .param("LS")
            .param("multi-prefix sasl")
            .build()
            .unwrap(),
    );
    assert!(msgs.unwrap().len() > 0)
}
