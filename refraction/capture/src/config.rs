use std::fs;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct IMAPServer {
    pub id: String,
    pub domain: String,
    pub port: u16,
    pub username: String,
    pub method: ConnectMethod
}

#[derive(Deserialize, Debug)]
pub enum ConnectMethod {
    STARTTLS,
    TLS
}

pub fn from_path(path: &str) -> IMAPServer {
    toml::from_str(&fs::read_to_string(path).unwrap()).unwrap()
}