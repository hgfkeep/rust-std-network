#[macro_use] extern crate nom;
use std::str;
use nom::{IResult};
use nom::error::ErrorKind;

/// Http 方法类型
#[derive(Debug)]
enum Method{
    GET,
    POST
}

///
/// 抽象表示 HTTP 协议请求，
/// 待解析的 HTTP 协议，例如：GET /home/ HTTP/1.1
#[derive(Debug)]
struct Request{
    method: Method,
    url: String,
    version: String,
}

///named! 定义解析宏，
/// return_error! 碰到第一个错误就结束解析，其中第一个参数是错误的类型
/// alt! 尝试一系列的parser，返回第一个成功的结果，
/// map! 将parser的结果映射到一个闭包中
named!(parse_method<&[u8], Method>, 
    return_error!(ErrorKind::Alt, alt!( map!(tag!("GET"), |_| Method::GET) | map!(tag!("POST"), |_| Method::POST) )));

/// ws! 处理空白字符，类似正则表达式\s,
/// do_parse!按顺序执行 一系列的子parser, 每个子parser间使用 >> 分割， 每个子parser何以定义属性，将结果赋值到属性
named!(parse_request<&[u8], Request>, ws!(
    do_parse!(
        method: parse_method 
        >> url: map_res!( take_until!(" "), str::from_utf8 ) 
        >> tag!("HTTP") 
        >> version: map_res!( take_until!("\r"), str::from_utf8)
        >> (Request{method, url: url.into(), version: version.into()})))
);

fn run_parser(input: &str){
    match parse_request(input.as_bytes()){
        IResult::Ok(ok) => println!("value: {:?}", ok ),
        IResult::Err(err) => eprintln!("{:?}", err),
    }
}


fn main(){
    let get = "GET /home/ HTTP/1.1\r\n";
    run_parser(get);
    let post = "POST /update/ HTTP/1.1\r\n";
    run_parser(post);
    let wrong = "WRONG /wrong/ HTTP/1.1\r\n";
    run_parser(wrong);
}