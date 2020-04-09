use std::env;
use std::net::{Ipv4Addr, UdpSocket, IpAddr};
use std::str;

fn main() {

    let mcast_group: Ipv4Addr = "234.2.2.2".parse().unwrap();
    let port: u16 = 6000;
    let any: Ipv4Addr = "0.0.0.0".parse().unwrap();
    let mut buffer = [0_u8; 1600];
    if env::args().count() > 1 {
        // 声明 服务端 通信使用的socket
        let socket = UdpSocket::bind((any, port)).expect("Faild to bind");

        // 服务端进入 组播组：join_multicast_v4 传入组播地址 和 加入组播地址使用的网口（即监听组播数据的IP）
        socket.join_multicast_v4(&mcast_group, &any).expect("Could not join boardcast group!");

        //服务端socket 监听本地的port 端口， 接收组播数据
        let (size, peer) = socket.recv_from(&mut buffer).expect("Failed to write to server");
        println!("{}:{}[{}]", peer, str::from_utf8(&buffer).expect("could write buffer as string!"), size);
    }else{
        // 客户端不需要服务端接收数据，监听端口直接配置成0
        let socket = UdpSocket::bind((any,0)).expect("Could not bind any port");
        // 客户端将消息发送到广播组IP的port端口上， 仅有服务端使用了同样port的socket，才能收到此广播消息
        socket.send_to("Hello World".as_bytes(), &(mcast_group, port)).expect("Could not write buffer");
    }
}
