use pest::Parser;

use super::{Grammar, Rule};

#[test]
fn test_nocrlf() {
    // should allow spaces...
    let input = String::from("asd asd\n");
    for char in input.chars() {
        let s = String::from(char);
        let res = Grammar::parse(Rule::nocrlf, &s);
        if s == "\n" {
            assert!(res.is_err())
        } else {
            let mut res = res.unwrap();
            assert_eq!(res.len(), 1);
            assert_eq!(res.next().unwrap().as_str(), s);
        }
    }
}

#[test]
fn test_nospcrlf() {
    // should skip spaces...
    let input = String::from("asd asd\n");
    for char in input.chars() {
        let s = String::from(char);
        println!("{s}");
        let res = Grammar::parse(Rule::nospcrlf, &s);
        if s == " " || s == "\n" {
            assert!(res.is_err());
        } else {
            let mut res = res.unwrap();
            assert_eq!(res.len(), 1);
            assert_eq!(res.next().unwrap().as_str(), s);
        }
    }
    //assert_eq!(res.len(), 3);
    //for (pair, char) in res.zip(input.chars()) {
    //    println!("{char}");
    //    assert_eq!(pair.as_str(), String::from(char));
    //}
}

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
    assert_eq!(pair.into_inner().len(), 0);
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

#[test]
fn test_tags() {
    let mut res = Grammar::parse(Rule::tags, "@id=123AB;rose").unwrap();
    let pair = res.next().unwrap();
    assert_eq!(pair.as_rule(), Rule::tags);

    // first tag "id=123AB"
    let mut inner = pair.into_inner();
    let pair = inner.next().unwrap();
    assert_eq!(pair.as_rule(), Rule::tag);

    {
        let mut inner = pair.into_inner();

        let pair = inner.next().unwrap();
        assert_eq!(pair.as_rule(), Rule::key);
        assert_eq!(pair.as_str(), "id");

        {
            let mut inner = pair.into_inner();
            assert_eq!(inner.len(), 1);

            let pair = inner.next().unwrap();
            assert_eq!(pair.as_rule(), Rule::key_chars);
            assert_eq!(pair.as_str(), "id");
        }

        let pair = inner.next().unwrap();
        assert_eq!(pair.as_rule(), Rule::escaped_value);
        assert_eq!(pair.as_str(), "123AB");
    }

    // second tag "rose" with empty value
    let pair = inner.next().unwrap();
    assert_eq!(pair.as_rule(), Rule::tag);

    {
        let mut inner = pair.into_inner();

        let pair = inner.next().unwrap();
        assert_eq!(pair.as_rule(), Rule::key);
        assert_eq!(pair.as_str(), "rose");

        // no value
        assert!(inner.next().is_none());
    }
}

#[test]
fn test_msg_cap_single() {
    let mut res = Grammar::parse(Rule::msg_cap, "CAP * LS :sasl").unwrap();
    let mut inner = res.next().unwrap().into_inner();

    let pair = inner.next().unwrap();
    assert_eq!(pair.as_rule(), Rule::cap_nick);
    assert_eq!(pair.as_str(), "*");

    let pair = inner.next().unwrap();
    assert_eq!(pair.as_rule(), Rule::cap_cmd);
    assert_eq!(pair.as_str(), "LS :sasl");

    {
        let mut inner = pair.into_inner();
        let pair = inner.next().unwrap();
        assert_eq!(pair.as_rule(), Rule::cap_ls);
        assert_eq!(pair.as_str(), "LS :sasl");

        {
            let mut inner = pair.into_inner();
            let pair = inner.next().unwrap();
            assert_eq!(pair.as_rule(), Rule::capability);
            assert_eq!(pair.as_str(), "sasl");
        }
    }
}

#[test]
fn test_msg_cap_multi_key_value_pair() {
    let input = "CAP * LS :multi-prefix sasl=PLAIN,EXTERNAL";
    let mut res = Grammar::parse(Rule::msg_cap, input).unwrap();
    let mut inner = res.next().unwrap().into_inner();

    let pair = inner.next().unwrap();
    assert_eq!(pair.as_rule(), Rule::cap_nick);
    assert_eq!(pair.as_str(), "*");

    let pair = inner.next().unwrap();
    assert_eq!(pair.as_rule(), Rule::cap_cmd);
    assert_eq!(pair.as_str(), "LS :multi-prefix sasl=PLAIN,EXTERNAL");

    {
        let mut inner = pair.into_inner();
        let pair = inner.next().unwrap();
        assert_eq!(pair.as_rule(), Rule::cap_ls);

        {
            let mut inner = pair.into_inner();
            let pair = inner.next().unwrap();
            assert_eq!(pair.as_rule(), Rule::capability);
            assert_eq!(pair.as_str(), "multi-prefix");
            {
                let mut inner = pair.into_inner();
                assert_eq!(inner.len(), 1);
                let pair = inner.next().unwrap();
                assert_eq!(pair.as_rule(), Rule::cap_key);
                assert_eq!(pair.as_str(), "multi-prefix");
            }

            let pair = inner.next().unwrap();
            assert_eq!(pair.as_rule(), Rule::capability);
            assert_eq!(pair.as_str(), "sasl=PLAIN,EXTERNAL");
            {
                let mut inner = pair.into_inner();
                assert_eq!(inner.len(), 2);

                let pair = inner.next().unwrap();
                assert_eq!(pair.as_rule(), Rule::cap_key);
                assert_eq!(pair.as_str(), "sasl");

                let pair = inner.next().unwrap();
                assert_eq!(pair.as_rule(), Rule::cap_values);
                assert_eq!(pair.as_str(), "PLAIN,EXTERNAL");

                {
                    let mut inner = pair.into_inner();
                    assert_eq!(inner.len(), 2);

                    let pair = inner.next().unwrap();
                    assert_eq!(pair.as_rule(), Rule::cap_value);
                    assert_eq!(pair.as_str(), "PLAIN");

                    let pair = inner.next().unwrap();
                    assert_eq!(pair.as_rule(), Rule::cap_value);
                    assert_eq!(pair.as_str(), "EXTERNAL");
                }
            }
        }
    }
}
