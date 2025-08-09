use std::io::{self, Write};

fn main() {
    let mut typer = UserTyper::new(CommandLineComputer);
    loop {
        typer.type_str(); // 实际上是 UserTyper::type_str(&mut typer)的语法糖
        println!("result: {}", typer.compute());
    }
}

trait Computer {
    fn compute(&self, expr: &str) -> i32;
}

struct CommandLineComputer;

impl Computer for CommandLineComputer {
    fn compute(&self, expr: &str) -> i32 {
        let mut num1 = String::new();
        let mut num2 = String::new();
        let mut operator: Option<char> = None;
        for ele in expr.trim().chars() {
            if ele.is_digit(10) {
                if operator.is_none() {
                    num1.push(ele);
                } else {
                    num2.push(ele);
                }
                continue;
            }

            match ele {
                '+' | '-' | '*' | '/' if operator.is_none() => operator = Some(ele),
                _ if ele.is_whitespace() => continue,
                _ => panic!("invalid operator {}", ele),
            }
        }
        if num1.is_empty() || num2.is_empty() || operator.is_none() {
            panic!("invalid expression {}", expr)
        }

        let num1 = num1.parse::<i32>().unwrap();
        let num2 = num2.parse::<i32>().unwrap();
        let operator = operator.unwrap();
        match operator {
            '+' => num1 + num2,
            '-' => num1 - num2,
            '*' => num1 * num2,
            '/' => num1 / num2,
            _ => unreachable!(),
        }
    }
}

struct UserTyper<T: Computer> {
    computer: T,
    expr: String,
}

impl<T: Computer> UserTyper<T> {
    fn new(computer: T) -> Self {
        Self {
            computer,
            expr: String::new(),
        }
    }

    fn type_str(&mut self) {
        self.expr.clear();
        println!("please type your expression: ");
        io::stdout().flush().unwrap();
        io::stdin()
            .read_line(&mut self.expr)
            .expect("read line failed");
    }

    fn compute(&self) -> i32 {
        self.computer.compute(&self.expr)
    }
}
