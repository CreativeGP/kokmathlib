/*
kokmathlib 
2018/07/10 (yyyy/mm/dd)

Library for algebraic operations!

Author: CreativeGP(@CreativeGP1)
 */

use structs::*;

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
fn token_to_string() {
    let t1 = Token::Scalar(1.0);
    let t2 = Token::Variable("x".to_string());

    assert_eq!(t1.to_string(), "1");
    assert_eq!(t2.to_string(), "x");
}
