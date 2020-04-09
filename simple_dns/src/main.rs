use std::net::ToSocketAddrs;
fn main() {
    // ToSocketAddrs trait 实现字符串str类型到SocketAddr类型迭代器转换
    let mut addrs = "baidu.com:9090".to_socket_addrs().unwrap();
    while let Some(socket) = addrs.next() {
        println!("socket: {}[ipv4={}]", socket.ip(), socket.is_ipv4());
    }
}
