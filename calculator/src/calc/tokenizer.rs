use std::{iter::Peekable, str::Chars};

use crate::calc::token::Token;

pub struct Tokenizer<'a> {
    expression: Peekable<Chars<'a>>,
    reached_end: bool,
    unexpected_char: Option<char>,
}

impl<'a> Tokenizer<'a> {
    pub fn new(expression: &'a str) -> Self {
        Self {
            expression: expression.chars().peekable(),
            reached_end: false,
            unexpected_char: None,
        }
    }

    pub fn get_unexpected_char(&self) -> Option<char> {
        self.unexpected_char
    }
}

//为Tokenizer实现迭代器，可以循环解析
impl<'a> Iterator for Tokenizer<'a> {
    // 定义关联类型，即每次 next()返回的类型
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        // 到达最后一个，直接返回
        if self.reached_end {
            return None;
        }

        let next_char = self.expression.next();

        match next_char {
            Some(chr) if chr.is_numeric() => {
                let mut number = String::from(chr);

                // 如果下一个字符是数字，返回Some，非数字返回None，跳出循环
                while let Some(next) = self.expression.next_if(|c| c.is_numeric()) {
                    number.push(next);
                }
                Some(Token::Number(number.parse().unwrap()))
            }

            Some(chr) if chr.is_whitespace() => {
                while let Some(_) = self.expression.next_if(|c| c.is_whitespace()) {}
                //递归，去除掉所有空白字符后，返回一个Token类型
                self.next()
            }

            Some('+') => Some(Token::Add),
            Some('-') => Some(Token::Sub),
            Some('*') => Some(Token::Multiply),
            Some('/') => Some(Token::Divide),
            Some('^') => Some(Token::Caret),
            Some('(') => Some(Token::LeftParen),
            Some(')') => Some(Token::RightParen),
            None => {
                self.reached_end = true;
                Some(Token::EOF)
            }
            Some(chr) => {
                self.unexpected_char = Some(chr);
                None
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use rust_decimal::dec;

    use super::*;

    #[test]
    fn test_tokenizer() {
        let tokenizer = Tokenizer::new("(1+   2)*3");
        assert_eq!(
            tokenizer.collect::<Vec<Token>>(),
            vec![
                Token::LeftParen,
                Token::Number(dec!(1)),
                Token::Add,
                Token::Number(dec!(2)),
                Token::RightParen,
                Token::Multiply,
                Token::Number(dec!(3) ),
                Token::EOF,
            ]
        );
    }
}
