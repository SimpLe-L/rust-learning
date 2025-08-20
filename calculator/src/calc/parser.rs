use crate::calc::{
    ast::Node,
    error::{CalcError, CalcResult},
    token::{Precedence, Token},
    tokenizer::Tokenizer,
};

//这里因为要保存Tokenizer，Tokenizer有生命周期，所以Parser也需要有生命周期
pub struct Parser<'a> {
    tokenizer: Tokenizer<'a>,
    current_token: Token,
}

impl<'a> Parser<'a> {
    pub fn new(expression: &'a str) -> CalcResult<Self> {
        let mut tokenizer = Tokenizer::new(expression);
        let current_token = tokenizer.next().ok_or_else(|| {
            CalcError::UnexpectedCharacter(tokenizer.get_unexpected_char().unwrap())
        })?;
        Ok(Parser {
            tokenizer,
            current_token,
        })
    }

    pub fn parse(&mut self) -> CalcResult<Node> {
        self.parse_expression(Precedence::Default)
    }
}

// 再次实现 Parser，这里面只实现私有的方法
impl<'a> Parser<'a> {
    fn next_token(&mut self) -> CalcResult<()> {
        self.current_token = self.tokenizer.next().ok_or_else(|| {
            CalcError::UnexpectedCharacter(self.tokenizer.get_unexpected_char().unwrap())
        })?;
        Ok(())
    }
    fn parse_expression(&mut self, precedence: Precedence) -> CalcResult<Node> {
        let mut expr = self.parse_number_or_expression()?;

        // 处理当前token的优先级
        while precedence < self.current_token.get_precedence() {
            expr = self.parse_binary_expression(expr)?;
        }
        Ok(expr)
    }

    fn parse_binary_expression(&mut self, left_expr: Node) -> CalcResult<Node> {
        match self.current_token {
            Token::Add => {
                self.next_token()?;
                let right_expr = self.parse_expression(Precedence::AddOrSubstract)?;
                Ok(Node::Add(Box::new(left_expr), Box::new(right_expr)))
            }
            Token::Sub => {
                self.next_token()?;
                let right_expr = self.parse_expression(Precedence::AddOrSubstract)?;
                Ok(Node::Sub(Box::new(left_expr), Box::new(right_expr)))
            }
            Token::Multiply => {
                self.next_token()?;
                let right_expr = self.parse_expression(Precedence::MultiplyOrDivide)?;
                Ok(Node::Multiply(Box::new(left_expr), Box::new(right_expr)))
            }
            Token::Divide => {
                self.next_token()?;
                let right_expr = self.parse_expression(Precedence::MultiplyOrDivide)?;
                Ok(Node::Divide(Box::new(left_expr), Box::new(right_expr)))
            }
            Token::Caret => {
                self.next_token()?;
                let right_expr = self.parse_expression(Precedence::Power)?;
                Ok(Node::Caret(Box::new(left_expr), Box::new(right_expr)))
            }
            _ => unreachable!(),
        }
    }

    fn parse_number_or_expression(&mut self) -> CalcResult<Node> {
        match self.current_token {
            Token::Number(n) => {
                self.next_token()?;
                Ok(Node::Number(n))
            }
            Token::Sub => {
                self.next_token()?;
                let expr = self.parse_expression(Precedence::Negative)?;
                Ok(Node::Negative(Box::new(expr)))
            }
            Token::LeftParen => {
                self.next_token()?;
                let expr = self.parse_expression(Precedence::Default)?;

                if self.current_token != Token::RightParen {
                    if self.current_token == Token::EOF {
                        return Err(CalcError::InvalidToken(String::from("不完整的运算表达式")));
                    }
                    return Err(CalcError::InvalidToken(format!(
                        "期望 ')', 但是发现 {}",
                        self.current_token
                    )));
                }
                self.next_token()?;
                Ok(expr)
            }
            _ => {
                if self.current_token == Token::EOF {
                    return Err(CalcError::InvalidToken(String::from("不完整的运算表达式")));
                }
                Err(CalcError::InvalidToken(format!(
                    "期望一个数字或表达式, 但是s发现 {}",
                    self.current_token
                )))
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rust_decimal::dec;

    #[test]
    fn test_parse_expression() {
        let mut parser = Parser::new("(1 +2  )*3").unwrap();
        assert_eq!(
            parser.parse(),
            Ok(Node::Multiply(
                Box::new(Node::Add(
                    Box::new(Node::Number(dec!(1))),
                    Box::new(Node::Number(dec!(2)))
                )),
                Box::new(Node::Number(dec!(3)))
            ))
        );
    }
}
