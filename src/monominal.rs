/*
kokmathlib 
2018/07/10 (yyyy/mm/dd)

Library for algebraic operations!

Author: CreativeGP(@CreativeGP1)
*/

use structs::*;

impl Monominal {
    // pub fn from_string(str: String) -> Monominal {
        
    // }
    
    pub fn to_string(&self) -> String {
        let mut res = "".to_string();

        {
            let mut chunks = self.numerator.iter().peekable();
            while let Some(chunk) = chunks.next() {
                match *chunk {
                    Token::Scalar(ref s) => res.insert_str(0, format!("{}*", s).as_str()),
                    Token::Variable(ref v) => res.push_str(format!("{}*", v).as_str()),
                    Token::Parenthesis(ref p) => res.push_str(format!("({})*", p.to_string()).as_str()),
                };

                if chunks.peek().is_none() {
                    res.pop();
                    // res.pop();
                    // res.pop();
                }
            }
        }

        if let Some(ref ex) = self.denominator {
            res = format!("{}/({})", res, ex.to_string());
        }

        res
    }

    pub fn to_latex(&self) -> String {
        let mut res = "".to_string();

        {
            let mut chunks = self.numerator.iter().peekable();
            while let Some(chunk) = chunks.next() {
                match *chunk {
                    Token::Scalar(ref s) => res.insert_str(0, format!("{}", s).as_str()),
                    Token::Variable(ref v) => res.push_str(&v[..1]),
                    Token::Parenthesis(ref p) => res.push_str(format!("({})", p.to_latex()).as_str()),
                };
            }
        }

        if let Some(ref ex) = self.denominator {
            res = format!("\\frac{{{}}}{{{}}}", res, ex.to_latex());
        }

        res
    }

    pub fn combine_scalar(&mut self) {
        let mut pum = 1.;

        {
            let mut diff = 0;
            for i in 0..self.numerator.len() {
                if let Token::Scalar(s) = self.numerator[i-diff] {
                    pum *= s;
                    self.numerator.remove(i-diff);
                    diff += 1;
                }
            }
        }

        self.numerator.push(Token::Scalar(pum));

        // TODO: 分母もやって約分までするべきなのかな？
    }
}


#[test]
fn monominal_combine_scalar() {
    let mut mono = Monominal {
        numerator: vec![
            Token::Scalar(2.),
            Token::Scalar(-3.),
            Token::Variable("x".to_string()),
            Token::Variable("y".to_string()),
        ],
        denominator: None
    };

    assert_eq!(format!("{:?}", mono), "Monominal { numerator: [Scalar(2), Scalar(-3), Variable(\"x\"), Variable(\"y\")], denominator: None }");

    mono.combine_scalar();

    assert_eq!(format!("{:?}", mono), "Monominal { numerator: [Variable(\"x\"), Variable(\"y\"), Scalar(-6)], denominator: None }");
}
