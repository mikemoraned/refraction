use std::env;

mod imap_session;
mod config;

fn main() {
    let config = config::from_path("config.toml");
    let password = env::var("IMAP_PASSWORD").unwrap();
    let args: Vec<String> = env::args().collect();
    let sequence_set = &args[1];

    let mut imap_session = 
        imap_session::open_session(&config.imap.domain, config.imap.port, &config.imap.username, &password).unwrap();

    let messages = imap_session.fetch(sequence_set, "RFC822").unwrap();
    if let Some(message) = messages.iter().next() {
        let body = message.body().expect("message did not have a body!");
        let body = std::str::from_utf8(body)
            .expect("message was not valid utf-8")
            .to_string();
        println!("message:");
        println!("{}", body);
    }

    imap_session::close_session(imap_session);
}
