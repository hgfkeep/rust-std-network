use std::io::{self, BufRead, BufReader, Result, Write};
use std::net::TcpStream;
use std::str;

fn main(){
    // 建立连接， 还可以使用TcpStream::connect_timeout(addr: &SocketAddr, timeout: Duration) 建立带响应超时的连接
    let mut stream = TcpStream::connect("127.0.0.1:9000").expect("Could not connect to server");
    loop {
        let mut input = String::new();
        let mut buffer: Vec<u8> = Vec::new();

        // 获取标准输入的消息
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read from stdin");

        //将标准输入输入的数据写入到stream，发送到服务端
        stream
            .write(input.as_bytes())
            .expect("Failed to write to server");

        // 按预期stream 中收到了echo server发送的响应，
        // 使用BufReader 获取stream中的响应
        let mut reader = BufReader::new(&stream);

        // 读取缓存区的数据，缓冲区是Vec 类型，根据需要自动增加，直到换行符
        reader
            .read_until(b'\n', &mut buffer)
            .expect("Could not read into buffer");
        
        // 将读取的数据输出
        print!(
            "{}",
            str::from_utf8(&buffer).expect("Could not write buffer as string")
        );
    }
}
