/*
kokmathlib 
2018/07/10 (yyyy/mm/dd)

Library for algebraic operations!

Author: CreativeGP(@CreativeGP1)
 */

use structs::*;

extern crate core;

extern crate regex;
use self::regex::Regex;

use std::str::FromStr;

impl FromStr for Token {
    type Err = String;
    
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // TODO: ここはToken::*が持っておくべきなのかもしれない
        let parenthesis_regex = Regex::new(r"\((?P<body>.*)\)").unwrap();
        let scalar_regex = Regex::new(r"(?:\d+\.)?\d+").unwrap();
        let variable_regex = Regex::new(r"\w+").unwrap();

        let result = if parenthesis_regex.is_match(s) {
            Ok(Token::Parenthesis(
                try!(Expression::from_str(
                    &parenthesis_regex.captures_iter(s).next().unwrap()["body"]
                ))))
        } else if scalar_regex.is_match(s) {
            Ok(Token::Scalar(
                match s.parse::<f64>() {
                    Ok(f) => f,
                    Err(e) => return Err(format!("String `{}` wasn't able to be parsed as float.", s)),
                }
            ))
        } else if variable_regex.is_match(s) {
            Ok(Token::Variable(s.to_string()))
        } else {
            Err("no matched.".to_string())
        };

        result
    }
}

impl Token {
    pub fn to_string(self: Token) -> String {
        match self {
            Token::Scalar(s) => format!("{}", s),
            Token::Variable(v) => v,
            Token::Parenthesis(p) => format!("({})", p.to_string()),
        }
    }
}

#[test]
fn to_string() {
    let t1 = Token::Scalar(1.0);
    let t2 = Token::Variable("x".to_string());

    assert_eq!(t1.to_string(), "1");
    assert_eq!(t2.to_string(), "x");
}

#[test]
fn from_string() {
    let t1 = "3".parse::<Token>().unwrap();
    let t2 = "x".parse::<Token>().unwrap();

    // TODO: test for parenthesises

    assert_eq!(format!("{:?}", t1), "Scalar(3)");
    assert_eq!(format!("{:?}", t2), "Variable(\"x\")");
}
