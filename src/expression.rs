/*
kokmathlib 
2018/07/10 (yyyy/mm/dd)

Library for algebraic operations!

Author: CreativeGP(@CreativeGP1)
*/ 

use structs::*;

impl Expression {
    pub fn from_str(str: &str) -> Result<Expression, String> {
        Err("coming soon...".to_string())
    }

    pub fn to_string(&self) -> String {
        let mut result = "".to_string();
        
        for m in self.0.iter() {
            if &m.to_string()[0..1] == "-" {
                result.push_str(format!("-{}", &m.to_string()[1..]).as_str());
            } else {
                result.push_str(format!("+{}", m.to_string()).as_str());
            }
        }

        if &result[..1] == "+" {
            (&result[1..]).to_string()
        } else {
            result
        }
    }

    pub fn to_latex(&self) -> String {
        let mut result = "".to_string();
        
        for m in self.0.iter() {
            if &m.to_latex()[0..1] == "-" {
                result.push_str(format!("-{}", &m.to_latex()[1..]).as_str());
            } else {
                result.push_str(format!("+{}", m.to_latex()).as_str());
            }
        }

        if &result[..1] == "+" {
            (&result[1..]).to_string()
        } else {
            result
        }
    }
}


#[test]
fn expression_to_string() {
    let mut exp = Expression(vec![
        Monominal {
            numerator: vec![
                Token::Scalar(2.),
                Token::Scalar(-3.),
                Token::Variable("x".to_string()),
                Token::Variable("y".to_string()),
            ],
            denominator: None
        },            
        Monominal {
            numerator: vec![
                Token::Scalar(-4.),
                Token::Variable("x".to_string()),
            ],
            denominator: None
        },            
        Monominal {
            numerator: vec![
                Token::Scalar(8.),
                Token::Scalar(2.1),
                Token::Variable("x".to_string()),
                Token::Variable("y".to_string()),
            ],
            denominator: None
        },
    ]);

    assert_eq!(exp.to_string(), "-3*2*x*y-4*x+2.1*8*x*y");
}

#[test]
fn complicated_expression_to_string() {
    let mut exp = Expression(vec![
        Monominal {
            numerator: vec![
                Token::Scalar(2.),
                Token::Parenthesis(Expression(vec![
                    Monominal {
                        numerator: vec![
                            Token::Scalar(-4.),
                            Token::Variable("x".to_string()),
                        ],
                        denominator: None
                    },            
                    Monominal {
                        numerator: vec![
                            Token::Scalar(8.),
                            Token::Scalar(2.1),
                            Token::Variable("x".to_string()),
                            Token::Variable("y".to_string()),
                        ],
                        denominator: None
                    }
                ])),
            ],
            denominator: None
        },
        Monominal {
            numerator: vec![
                Token::Variable("x".to_string()),
                Token::Variable("y".to_string()),
            ],
            denominator: Some(Box::new(
                Expression(vec![
                    Monominal {
                        numerator: vec![
                            Token::Scalar(2.),
                            Token::Parenthesis(Expression(vec![
                                Monominal {
                                    numerator: vec![
                                        Token::Scalar(-4.),
                                        Token::Variable("x".to_string()),
                                    ],
                                    denominator: None
                                },            
                                Monominal {
                                    numerator: vec![
                                        Token::Scalar(8.),
                                        Token::Variable("x".to_string()),
                                        Token::Variable("y".to_string()),
                                    ],
                                    denominator: None
                                }
                            ])),
                        ],
                        denominator: None
                    },
                    Monominal {
                        numerator: vec![Token::Variable("a".to_string())], denominator: None
                    },
                ]))),
        },
    ]);

    println!("{}", exp.to_string());

    assert_eq!(exp.to_string(), "2*(-4*x+2.1*8*x*y)+x*y/(2*(-4*x+8*x*y)+a)");
}
