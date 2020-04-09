#[macro_use]
extern crate serde_derive;
use serde::{Deserialize, Serialize};
use std::io::Read;

#[derive(Serialize, Deserialize, Debug)]
struct ServerConfig {
    workers: Vec<char>,
    ignore: bool,
    auth_server: Option<String>,
}

fn json(config: &ServerConfig) {
    let serialized = serde_json::to_string(config).unwrap();
    println!("json serialized: {}", serialized);
    let deserialized: ServerConfig = serde_json::from_str(&serialized).unwrap();
    println!("json deserialized: {:?}", deserialized);
}

fn yaml(config: &ServerConfig) {
    let serialized = serde_yaml::to_string(config).unwrap();
    println!("yaml serialized: {}", serialized);
    let deserialized: ServerConfig = serde_yaml::from_str(&serialized).unwrap();
    println!("yaml deserialized: {:?}", deserialized);
}

fn main() {
    let config: ServerConfig = ServerConfig {
        workers: vec!['a', 'b', 'c'],
        ignore: true,
        auth_server: Some("auth.server.io".to_string()),
    };
    yaml(&config);
    json(&config);
}
