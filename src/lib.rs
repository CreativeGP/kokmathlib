/*
kokmathlib 
2018/07/10 (yyyy/mm/dd)

Library for algebraic operations!

Author: CreativeGP(@CreativeGP1)
*/ 

/*
【NOTE 数式の情報を保存する構造体たちの構成】

数式をRustの構造体で表していく。
数式にはたくさんの構成要素が在る（括弧、函数、絶対値記号、行列、二項係数、etc...）
これらは単項式を構成する要素になることが出来るため、
これらをまとめてTokenとして定義して、(簡単に言うと)単項式をTokenのリストと定義する。
また、式は単項式を１次演算子で繋いだものになるはずであるため、Monominalのリストと定義する。

計算が便利なので、単項式は全て分数で表すことが出来るので分数として定義する。
実際には、分子はTokenのリスト、分母はExpressionのリストとして定義する。
 */


#[derive(Debug)]
pub enum Token {
    Scalar(f64),
    Variable(String),
    Parenthesis(Expression),
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

#[derive(Debug)]
pub struct Monominal {
    pub numerator: Vec<Token>,
    pub denominator: Option<Box<Expression>>,
}

impl Monominal {
    pub fn to_string(&self) -> String {
        let mut res = "".to_string();

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

        // if let Some(ex) = self.denominator {
        //     res = format!("{} / ({})", res, ex.to_string());
        // }

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

#[derive(Debug)]
pub struct Expression(pub Vec<Monominal>);

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

        result
    }
}



#[derive(Debug)]
pub struct Equation {
    left: Expression,
    right: Expression,
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }

    #[test]
    fn token_to_string() {
        let t1 = Token::Scalar(1.0);
        let t2 = Token::Variable("x".to_string());

        assert_eq!(t1.to_string(), "1");
        assert_eq!(t2.to_string(), "x");
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

    #[test]
    fn monominal_to_string() {
        let mut mono = Monominal {
            numerator: vec![
                Token::Scalar(2.),
                Token::Scalar(-3.),
                Token::Variable("x".to_string()),
                Token::Variable("y".to_string()),
            ],
            denominator: None
        };

        assert_eq!(mono.to_string(), "-3*2*x*y");
        mono.combine_scalar();
        assert_eq!(mono.to_string(), "-6*x*y");
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

        println!("{}", exp.to_string());
    }
}
