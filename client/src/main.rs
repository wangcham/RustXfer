use std::env;
use std::fs;
use std::io::{self,Write};
use std::net::TcpStream;
use std::process;
use std::path::Path;
use std::string;

fn main(){
    println!("请直接输入文件路径或输入quit退出:");
    loop{
        print!(">>>");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let input = input.trim();
        match input.trim() {
            "RIP" => print!("Matthew Perry\n"),
            "q" => break,
             _ => check(&input),
        }
    }

}
// check if the file exists
fn check(file_path:&str){
    let path = Path::new(file_path);
    if path.exists(){
        print!("存在此文件\n");
        enterinfo(file_path);
    }else{
        print!("文件不存在\n");
    }
}

fn enterinfo(file_path: &str) {
    print!("请输入ip地址和端口号:\n");
    let mut addr = String::new();
    io::stdin().read_line(&mut addr).unwrap();
    let addr = addr.trim();

    // 读取文件内容
    let contents = fs::read(file_path).expect("无法读取文件");

    match TcpStream::connect(addr) {
        Ok(mut stream) => {
            stream.write_all(&contents)
                .expect("无法发送文件");
            println!("文件已发送至服务器");
        }
        Err(e) => {
            eprintln!("无法连接到服务器: {}", e);
        }
    }
}


