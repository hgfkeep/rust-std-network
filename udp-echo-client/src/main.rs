use std::net::UdpSocket;
use std::{io, str};

fn main() {
    // 设置UDP客户端接收服务端Response的端口， 请求数据时会将接收端口号一起发送给服务端
    // 与TCP客户端不同， TCP是面向连接的可靠通信，TCP客户端连接TCP服务端时，Stream会自动选择随机的端口，与服务端保持稳定的连接
    let socket = UdpSocket::bind("127.0.0.1:9001").expect("Cloud not bind client!");

    // UDP 客户端连接的server的地址
    socket
        .connect("127.0.0.1:9000")
        .expect("Cloud not connect to server");

    loop {
        let mut input = String::new();
        let mut buffer = [0_u8; 150000];
        io::stdin().read_line(&mut input).expect("Failed to read");

        // 发送数据到UDP 服务端
        socket
            .send(input.as_bytes())
            .expect("Failed to send to server");

        // 接收UDP服务端数据
        socket
            .recv_from(&mut buffer)
            .expect("Could not read into buffer");
        println!(
            "{}",
            str::from_utf8(&buffer).expect("could write buffer as string")
        );
    }
}
