/*
kokmathlib 
2018/07/10 (yyyy/mm/dd)

Library for algebraic operations!

Author: CreativeGP(@CreativeGP1)
*/

use structs::*;

use std::str::FromStr;

impl FromStr for Monominal {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut result = Monominal {numerator: vec![], denominator: None};
        let mut p = s.len();
        let mut level = 0;

        let mut exp = "".to_string();

        // NOTE: 逆からパースしていくためリスト逆になる
        // 嫌ならリスト作った後にreverseして
        for (i, c) in s.char_indices().rev() {
            match c {
                '*' if level == 0 => {
                    result.numerator.push((&s[i+1..p]).parse::<Token>()?);
                    p = i;
                },
                '/' if level == 0 => {
                    exp.push_str(format!("({})*", &s[i+1..p]).as_str());
                    p = i;
                },
                '(' => level += 1,
                ')' => level -= 1,
                _ => (),
            }
        }

        if &s[..p] != "" {
            result.numerator.push((&s[..p]).parse::<Token>()?);
        }

        if exp != "" {
            exp.pop();
            result.denominator = Some(Box::new(exp.parse::<Expression>()?));
        }

        Ok(result)
    }
}

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

    // TODO: もっと一般化すべき -1のかけ算 -> 数値のかけ算 -> Monominalのかけ算
    pub fn zero_invert(&mut self) {
        let mut found = false;

        for i in 0..self.numerator.len() {
            if let Token::Scalar(ref mut s) = self.numerator[i] {
                *s *= -1.;
                found = true;
                break;
            }
        }

        if !found {
            self.numerator.push(Token::Scalar(-1.));
        }
    }
}


#[test]
fn combine_scalar() {
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

#[test]
fn zero_invert() {
    let mut m1 = Monominal {
        numerator: vec![
            Token::Scalar(2.),
            Token::Variable("x".to_string()),
            Token::Variable("y".to_string()),
        ],
        denominator: None
    };
    let mut m2 = Monominal {
        numerator: vec![
            Token::Variable("x".to_string()),
            Token::Variable("y".to_string()),
        ],
        denominator: None
    };

    m1.zero_invert();
    m2.zero_invert();
    
    assert_eq!(format!("{}", m1.to_string()), "-2*x*y");
    assert_eq!(format!("{}", m2.to_string()), "-1*x*y");
}

#[test]
fn from_string() {
    let m1 = "x*y".parse::<Monominal>().unwrap();
    let m2 = "3.8*7.2*y".parse::<Monominal>().unwrap();
    let m3 = "3*(a+b)".parse::<Monominal>().unwrap();
    let m3 = "a/c/3".parse::<Monominal>().unwrap();

    println!("{:?}", m3);

    assert_eq!(format!("{:?}", m1),
               "Monominal { numerator: [Variable(\"y\"), Variable(\"x\")], denominator: None }");
    assert_eq!(format!("{:?}", m2),
               "Monominal { numerator: [Variable(\"y\"), Scalar(7.2), Scalar(3.8)], denominator: None }");
}
