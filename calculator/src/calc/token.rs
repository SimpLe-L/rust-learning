use std::fmt::{Display, Formatter};

use rust_decimal::Decimal;

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Token {
    Add,
    Sub,
    Multiply,
    Divide,
    Caret,
    LeftParen,
    RightParen,
    Number(Decimal),
    EOF,
}

//为token实现获取优先级的方法
impl Token {
    pub fn get_precedence(&self) -> Precedence {
        use Precedence::*;
        use Token::*;

        match self {
            Add | Sub => AddOrSubstract,
            Multiply | Divide => MultiplyOrDivide,
            Caret => Power,
            _ => Default,
        }
    }
}

impl Display for Token {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        use Token::*;
        match self {
            Add => write!(f, "+"),
            Sub => write!(f, "-"),
            Multiply => write!(f, "*"),
            Divide => write!(f, "/"),
            Caret => write!(f, "^"),
            LeftParen => write!(f, "("),
            RightParen => write!(f, ")"),
            Number(n) => write!(f, "{}", n),
            EOF => write!(f, "EOF"),
        }
    }
}

//定义优先级结构
#[derive(Debug, Copy, Clone, PartialOrd, PartialEq)]
pub enum Precedence {
    Default,
    AddOrSubstract,
    MultiplyOrDivide,
    Power,
    Negative,
}
