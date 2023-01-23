use std::{net::TcpStream, io::{Read, Write}};
use imap::Session;

pub fn open_session(domain: &str, port: u16, username: &str, password: &str) 
    -> imap::error::Result<Session<TcpStream>> {
    let stream = TcpStream::connect((domain, port)).unwrap();
    let client = imap::Client::new(stream);

    let mut session = client
        .login(username, password)
        .map_err(|e| e.0)?;

    session.examine("INBOX")?;

    Ok(session)
}

pub fn close<T: Read + Write>(mut session: Session<T>) {
    session.logout().unwrap();
}