use pest::Parser;

use super::{Grammar, Rule};

#[test]
fn test_middle() {
    let mut res = Grammar::parse(Rule::middle, "asd").unwrap();
    let pair = res.next().unwrap();
    println!("{pair:?}");
    assert_eq!(pair.as_rule(), Rule::middle);
    assert_eq!(pair.as_str(), "asd");
    assert_eq!(pair.into_inner().len(), 0);
}

#[test]
fn test_trailing() {
    let mut res = Grammar::parse(Rule::trailing, ":asd xyz").unwrap();
    let pair = res.next().unwrap();
    println!("{pair:?}");
    assert_eq!(pair.as_rule(), Rule::trailing);
    assert_eq!(pair.as_str(), ":asd xyz");

    let mut inner = pair.into_inner();
    assert_eq!(inner.len(), 1);

    let pair = inner.next().unwrap();
    assert_eq!(pair.as_rule(), Rule::trailing_inner);
    assert_eq!(pair.as_str(), "asd xyz");
}

#[test]
fn test_simple_command() {
    let mut res = Grammar::parse(Rule::command, "PING").unwrap();
    let pair = res.next().unwrap();
    println!("{pair:?}");
    assert_eq!(pair.as_rule(), Rule::command);
    assert_eq!(pair.as_str(), "PING");
    let mut inner = pair.into_inner();
    assert_eq!(inner.len(), 0);
}

#[test]
fn test_3digit_command() {
    let mut res = Grammar::parse(Rule::command, "001").unwrap();
    let pair = res.next().unwrap();
    println!("{pair:?}");
    assert_eq!(pair.as_rule(), Rule::command);
    assert_eq!(pair.as_str(), "001");
    let mut inner = pair.into_inner();
    assert_eq!(inner.len(), 1);
    let pair = inner.next().unwrap();
    assert_eq!(pair.as_rule(), Rule::digit3);
    assert_eq!(pair.as_str(), "001");
}

#[test]
fn test_invalid_single_digit_command() {
    let res = Grammar::parse(Rule::command, "1");
    assert!(res.is_err());
}
