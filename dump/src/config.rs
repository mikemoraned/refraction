use serde::Deserialize;
use std::fs;

#[derive(Deserialize)]
pub struct Config {
    pub sources: Option<Vec<IMAP>>,
    pub feeds: Option<Vec<FeedConfig>>
}

#[derive(Deserialize, Debug)]
pub struct IMAP {
    pub id: String,
    pub domain: String,
    pub port: u16,
    pub username: String,
}

#[derive(Deserialize, Debug, Clone)]
pub struct FeedConfig {
    pub id: String,
    pub title: String,
    pub email: String
}

pub fn from_path(path: &str) -> Config {
    toml::from_str(&fs::read_to_string(path).unwrap()).unwrap()
}