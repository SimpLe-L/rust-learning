use rust_decimal::Decimal;

use crate::calc::error::CalcResult;

mod ast;
mod error;
mod parser;
mod token;
mod tokenizer;

pub fn calculate(expression: &str) -> CalcResult<Decimal> {
    let mut parser = parser::Parser::new(expression)?;
    let ast = parser.parse()?;
    Ok(ast.evaluate())
}

#[cfg(test)]
mod tests {
    use rust_decimal::dec;

    use super::*;

    #[test]
    fn test_calculate() {
        assert_eq!(calculate("1 + 2").unwrap(), dec!(3));
        assert_eq!(calculate("1 + 2 * 3 + (-1)").unwrap(), dec!(6));
    }
}
