use rust_decimal::{Decimal, MathematicalOps};

#[derive(Debug, PartialEq, Clone)]
pub enum Node {
    Add(Box<Node>, Box<Node>),
    Sub(Box<Node>, Box<Node>),
    Multiply(Box<Node>, Box<Node>),
    Divide(Box<Node>, Box<Node>),
    Caret(Box<Node>, Box<Node>),
    Negative(Box<Node>),
    Number(Decimal),
}

impl Node {
    pub fn evaluate(&self) -> Decimal {
        use Node::*;
        match self {
            Add(left, right) => left.evaluate() + right.evaluate(),
            Sub(left, right) => left.evaluate() - right.evaluate(),
            Multiply(left, right) => left.evaluate() * right.evaluate(),
            Divide(left, right) => left.evaluate() / right.evaluate(),
            Caret(left, right) => left.evaluate().powd(right.evaluate()),
            Negative(expr) => -expr.evaluate(),
            Number(n) => *n,
        }
    }
}

#[cfg(test)]
mod tests {
    use rust_decimal::dec;

    use crate::calc::ast::Node;

    #[test]
    fn test_evaluate() {
        let expr = Node::Add(
            Box::new(Node::Number(dec!(1))),
            Box::new(Node::Number(dec!(2))),
        );
        assert_eq!(expr.evaluate(), dec!(3));
    }
}
