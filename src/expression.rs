/*
kokmathlib 
2018/07/10 (yyyy/mm/dd)

Library for algebraic operations!

Author: CreativeGP(@CreativeGP1)
*/ 

use structs::*;

use std::str::FromStr;

impl FromStr for Expression {
    type Err = String;
    
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut result = Expression(vec![]);
        let mut p = s.len();
        let mut level = 0;

        let mut exp = "".to_string();
        
        for (i, c) in s.char_indices().rev() {
            match c {
                '+' if level == 0 => {
                    result.0.push((&s[i+1..p]).parse::<Monominal>()?);
                    p = i;
                },
                '-' if level == 0 => {
                    result.0.push((&s[i+1..p]).parse::<Monominal>()?);
                    result.0.last_mut().unwrap().zero_invert();
                    p = i;
                },
                '(' => level += 1,
                ')' => level -= 1,
                _ => (),
            }
        }

        if &s[..p] != "" {
            result.0.push((&s[..p]).parse::<Monominal>()?);
        }

        Ok(result)
    }
}

impl Expression {

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

#[test]
fn form_string() {
    // let m1 = "a+b".parse::<Expression>().unwrap();
    // let m2 = "a-b*10".parse::<Expression>().unwrap();
    let m3 = "2*(-4*x+2.1*8*x*y)+x*y/(2*(-4*x+8*x*y)+a)".parse::<Expression>().unwrap();

    println!("{:?}", m3);
}
