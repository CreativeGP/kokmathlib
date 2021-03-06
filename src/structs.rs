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

#[derive(Debug)]
pub struct Monominal {
    pub numerator: Vec<Token>,
    pub denominator: Option<Box<Expression>>,
}

#[derive(Debug)]
pub struct Expression(pub Vec<Monominal>);

#[derive(Debug)]
pub struct Equation {
    left: Expression,
    right: Expression,
}
