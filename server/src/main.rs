use std::fs::File;
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::thread;

fn handle_client(mut stream: TcpStream) {
    let mut buffer = [0; 512]; // 创建一个缓冲区来存储接收到的数据
    let mut file = File::create("received_file").unwrap(); // 创建一个文件来保存接收到的数据

    loop {
        let nbytes = stream.read(&mut buffer).unwrap(); // 从流中读取数据
        if nbytes == 0 {
            break; // 如果没有读取到数据，那么就跳出循环
        }
        file.write_all(&buffer[..nbytes]).unwrap(); // 将数据写入文件
    }

    println!("文件已接收并保存到本地");
}

fn main() {
    let listener = TcpListener::bind("localhost:12345").unwrap(); // 在指定的地址和端口上创建一个监听器

    for stream in listener.incoming() { // 对于每一个连接到服务器的客户端
        match stream {
            Ok(stream) => { // 如果连接成功
                thread::spawn(|| { // 创建一个新的线程来处理客户端
                    handle_client(stream); // 处理客户端
                });
            }
            Err(e) => { // 如果连接失败
                eprintln!("无法连接到客户端: {}", e);
            }
        }
    }
}

