use std::net::UdpSocket;
use std::thread;

fn main() {
    // 绑定端口
    let socket = UdpSocket::bind("0.0.0.0:9000").expect("Cannot bind port");

    loop {
        let mut buf = [0; 1500];

        // 克隆socket，闭包中需要move，转移所有权
        let sock = socket.try_clone().expect("Failed to Clone socket()");
        match socket.recv_from(&mut buf) {
            Ok((size, src)) => {
                thread::spawn(move || {
                    println!("Handing connection from {}", src);
                    // UDP不是面向连接的， 发送数据时，需要指定接收端地址，
                    sock.send_to(&buf[..size], &src)
                        .expect("Failed to send response");
                });
            }
            Err(e) => {
                eprintln!("Could recieve a message: {}", e);
            }
        }
    }
}
