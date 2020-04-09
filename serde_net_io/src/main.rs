#[macro_use]
extern crate serde_derive;
extern crate log;
use clap::{App, Arg};
use env_logger;
use log::{debug, error};
use serde_json;
use std::env;
use std::net::{TcpListener, TcpStream};
use std::num::ParseIntError;
use std::str;
use std::{
    io::{self, stdin, BufRead, BufReader, Write},
    thread,
};

#[derive(Serialize, Deserialize, Debug)]
struct Point3D(i32, i32, i32);

// 计算3D坐标系中，任意点和原点距离
fn cal(point: &Point3D) -> f64 {
    let x: f64 = (point.0.pow(2) + point.1.pow(2) + point.2.pow(2)) as f64;
    x.sqrt()
}

/// 客户端处理函数
fn handle_client(stream: TcpStream) -> io::Result<()> {
    debug!("Incomming connection from {}", stream.peer_addr()?);

    let mut data: Vec<u8> = Vec::new();

    // 将stream 转为BufReader
    let mut reader = BufReader::new(stream);

    loop {
        // 清理缓存
        data.clear();

        //读取到消息结束符号
        let read_bytes = reader
            .read_until(b'\n', &mut data)
            .expect("read from peer error");
        if read_bytes == 0 {
            return Ok(());
        }
        // 从消息中反序列化json
        let point: Point3D = serde_json::from_slice(&data)?;
        let value: f64 = cal(&point);

        //将计算结果返回客户端
        write!(reader.get_mut(), "{}\n", value)?;
    }
}

fn server() {
    //绑定监听端口
    let socket = TcpListener::bind("0.0.0.0:9000").expect("Could not bind port 9000");

    // 处理每一个连接进来的客户端
    for stream in socket.incoming() {
        match stream {
            Ok(stream) => {
                thread::spawn(move || {
                    handle_client(stream).unwrap_or_else(|err| {
                        error!("err: {:?}", err);
                    });
                });
            }
            Err(e) => {
                error!("Err: {:?}", e);
            }
        }
    }
}

/// 处理客户端逻辑， 向leader 发送json格式的3D坐标系中的点坐标
fn client() -> Result<(), ParseIntError> {
    // 建立连接
    let mut stream = TcpStream::connect("127.0.0.1:9000").expect("Connect connect server!");

    loop {
        let mut input = String::new();
        stdin()
            .read_line(&mut input)
            .expect("cannot read from stdin!");
        let p: Vec<&str> = input.trim_matches('\n').split(",").collect();
        if p.len() == 3 {
            // 生成坐标
            let point: Point3D = Point3D(p[0].parse()?, p[1].parse()?, p[2].parse()?);

            //发送坐标数据
            stream
                .write_all(
                    serde_json::to_string(&point)
                        .expect("cannot convert to json!")
                        .as_bytes(),
                )
                .expect("Connot write to server!");

            //发送消息结束符号
            stream
                .write_all("\n".as_bytes())
                .expect("Connect write to server!");

            // 读取结果
            let mut buf: Vec<u8> = Vec::new();
            let mut reader = BufReader::new(&stream);
            //读取到消息结束符号
            reader
                .read_until(b'\n', &mut buf)
                .expect("Could not read from client stream!");
            let input = str::from_utf8(&buf).expect("Could write buffer!");
            println!("Result: {}", input);
        } else {
            error!("client input error, input should like 1,2,3");
        }
    }
}

fn main() {
    // 初始化日志生成器
    env_logger::init();

    // 获取命令行参数
    let matches = App::new("serde_netio")
        .about(env!("CARGO_PKG_DESCRIPTION"))
        .author(env!("CARGO_PKG_AUTHORS"))
        .version(env!("CARGO_PKG_VERSION"))
        .arg(
            Arg::with_name("server")
                .short("s")
                .required_unless("client"),
        )
        .arg(
            Arg::with_name("client")
                .short("c")
                .required_unless("server"),
        )
        .get_matches();

    if matches.occurrences_of("server") > 0 {
        server();
    } else if matches.occurrences_of("client") > 0 {
        client().expect("cannot parse string to i32");
    }
}
