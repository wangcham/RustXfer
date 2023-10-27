use std::io::{self, Write};

fn main() {
    loop {
        print!("请输入命令: ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();

        match input.trim() {
            "hello" => println!("你好！"),
            "quit" => break,
            _ => println!("未知命令"),
        }
    }
}

