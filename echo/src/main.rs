use std::io::{Result, Write, Read};
use std::net::{TcpListener, TcpStream};
use std::thread;


fn handle_client(mut stream: TcpStream) -> Result<()> {
    println!("Incoming connection from {}", stream.peer_addr()? );

    let mut buf = [0_u8; 512];
    loop{
        // 读取流中的数据，并返回读取的长度
        let bytes_read = stream.read(&mut buf)?;

        // 读取为0，那么数据流结束
        if bytes_read == 0{
            return Ok(());
        }

        //将读入的数据写入到stream中，输出到客户端，完成echo的响应
        stream.write(&buf[..bytes_read])?;
    }
}

fn main() {
    //TcpListener 表示一个套接字，用于监听来自客户机的接入，本地地址设置为 0.0.0.0 会告诉内核将这个套接字绑定到这个主机上所有可用的接口上
    let listener = TcpListener::bind("0.0.0.0:9000").expect("Could not bind");

    // listener 上的 incoming 方法返回已连接到服务器的流上的迭代器。
    for stream in listener.incoming(){
        match stream{
            Err(e) => eprintln!("failed: {}", e),
            Ok(stream) =>{
                // 处理每个连接的数据流
                thread::spawn(move || {
                    handle_client(stream).unwrap_or_else(|error|eprintln!("{:?}", error));
                });
            }
        }
    }
}
