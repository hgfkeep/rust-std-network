#[macro_use]
extern crate log;
extern crate env_logger;
use log::{debug, info};

#[derive(Debug, PartialEq)]
struct KubeConfig{
    port: u8,
    healthz_port: u8,
    max_pos: u8
}

fn main() {
    env_logger::init();


}
