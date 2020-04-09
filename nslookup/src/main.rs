extern crate dns_lookup;

use std::io;
use std::net::IpAddr;

/// 通过域名获取ip列表迭代器
fn domain_ips(domain: &'static str) -> io::Result<Vec<IpAddr>> {
    return dns_lookup::lookup_host(domain);
}

/// 通过 ip 查找对应的主机名
fn ip_domain(ip: &'static str) -> io::Result<String> {
    match ip.parse() {
        Ok(ip) => dns_lookup::lookup_addr(&ip),
        Err(e) => Err(io::Error::from(io::ErrorKind::AddrNotAvailable)),
    }
}

fn main() {
    let baidu = "baidu.com";
    let ips: Vec<IpAddr> = domain_ips(&baidu).expect("nslookup error!");
    print!("{}:", baidu);
    for ip in ips {
        print!("{},", ip);
    }
    println!("",);

    let ip = "46.82.174.69";
    println!(
        "{} belong to {}",
        ip,
        ip_domain(&ip).expect("ip reverse to domain error!")
    );
}
