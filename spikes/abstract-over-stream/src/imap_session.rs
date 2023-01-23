use std::{net::TcpStream, io::{Read, Write}};
use imap::Session;
use native_tls::TlsStream;

pub fn open_starttls_session(domain: &str, port: u16, username: &str, password: &str) 
    -> imap::error::Result<Session<TlsStream<TcpStream>>> {
    let tls = native_tls::TlsConnector::builder().build().unwrap();

    let client = 
        imap::connect_starttls((domain, port), domain, &tls).unwrap();

    let mut imap_session = client
        .login(username, password)
        .map_err(|e| e.0)?;

    imap_session.examine("INBOX")?;

    Ok(imap_session)
}

pub fn open_tls_session(domain: &str, port: u16, username: &str, password: &str) 
    -> imap::error::Result<Session<TlsStream<TcpStream>>> {
    let tls = native_tls::TlsConnector::builder().build().unwrap();

    let client = 
        imap::connect((domain, port), domain, &tls).unwrap();

    let mut imap_session = client
        .login(username, password)
        .map_err(|e| e.0)?;

    imap_session.examine("INBOX")?;

    Ok(imap_session)
}

pub fn close<T: Read + Write>(mut session: Session<T>) {
    session.logout().unwrap();
}