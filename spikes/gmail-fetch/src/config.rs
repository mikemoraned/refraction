use serde::Deserialize;
use std::fs;

#[derive(Deserialize)]
pub struct Config {
    pub imap: IMAP,
}

#[derive(Deserialize)]
pub struct IMAP {
    pub domain: String,
    pub port: u16,
    pub username: String,
}

pub fn from_path(path: &str) -> Config {
    toml::from_str(&fs::read_to_string(path).unwrap()).unwrap()
}