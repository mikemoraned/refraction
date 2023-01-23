use std::env;
use std::io::{Read, Write};
use imap::Session;

mod imap_session;
mod config;

fn main() {
    let config = config::from_path("config.toml");
    let password = env::var("IMAP_PASSWORD").unwrap();
    let args: Vec<String> = env::args().collect();
    let sequence_set = &args[1];

    if config.imap.tls {
        let mut imap_session = 
            imap_session::open_tls_session(&config.imap.domain, config.imap.port, &config.imap.username, &password).unwrap();

        fetch_message(&mut imap_session, sequence_set);

        imap_session::close(imap_session);
    }
    else {
        let mut imap_session = 
            imap_session::open_session(&config.imap.domain, config.imap.port, &config.imap.username, &password).unwrap();

        fetch_message(&mut imap_session, sequence_set);

        imap_session::close(imap_session);
    }
}

pub fn fetch_message<T: Read + Write>(imap_session: &mut Session<T>, sequence_set: &str) {
    let messages = imap_session.fetch(sequence_set, "RFC822").unwrap();
    if let Some(message) = messages.iter().next() {
        let body = message.body().expect("message did not have a body!");
        let body = std::str::from_utf8(body)
            .expect("message was not valid utf-8")
            .to_string();
        println!("message:");
        println!("{}", body);
    }
}
