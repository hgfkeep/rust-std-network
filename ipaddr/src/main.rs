#![feature(ip)]

use std::net::IpAddr;
use std::net::SocketAddr;

fn main() {
    let ipv4:IpAddr = "127.0.0.1".parse().unwrap();
    assert!(ipv4.is_loopback());

    let global: IpAddr = IpAddr::from([0, 0, 0x1c9, 0, 0, 0xafc8, 0, 0x1]);
    assert!(global.is_global());

    // 通过IPv4形式的字符串构造SocketAddr
    let local_sa: SocketAddr = "127.0.0.1:80".parse().unwrap();
    assert!(local_sa.is_ipv4());

    // 通过使用IPv6地址构造SocketAddr
    let global_sa = SocketAddr::new(global, 80u16);
    assert!(global_sa.is_ipv6());
}
