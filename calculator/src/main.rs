mod calc;

fn main() {
    println!("欢迎测试计算器！");
    println!("输入表达式后回车计算结果！");
    println!("输入Q退出！");

    loop {
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();
        if input.trim() == "q" {
            println!("退出!");
            break;
        } else if input.is_empty() {
            continue;
        }
        match calc::calculate(&input) {
            Ok(result) => println!("计算结果为--{}", result),
            Err(err) => println!("计算错误--{}", err),
        }
    }
}
