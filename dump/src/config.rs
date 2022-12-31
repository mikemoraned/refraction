use serde::Deserialize;
use std::fs;

#[derive(Deserialize)]
pub struct Config {
    pub imap: IMAP,
    pub feeds: Option<Vec<FeedConfig>>
}

#[derive(Deserialize)]
pub struct IMAP {
    pub domain: String,
    pub port: u16,
    pub username: String,
}

#[derive(Deserialize, Debug)]
pub struct FeedConfig {
    pub id: String,
    pub title: String,
    pub email: String
}

pub fn from_path(path: &str) -> Config {
    toml::from_str(&fs::read_to_string(path).unwrap()).unwrap()
}