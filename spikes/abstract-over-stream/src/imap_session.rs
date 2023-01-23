use std::{net::TcpStream, io::{Read, Write}};
use imap::Session;
use native_tls::TlsStream;

pub trait ReadWrite: Read + Write {}
impl<T: Read + Write> ReadWrite for T {}

pub fn open_session(domain: &str, port: u16, username: &str, password: &str) 
    -> imap::error::Result<Session<Box<dyn ReadWrite>>> {
    let stream : Box<dyn ReadWrite> = Box::new(TcpStream::connect((domain, port)).unwrap());
    let client = imap::Client::new(stream);

    let mut session = client
        .login(username, password)
        .map_err(|e| e.0)?;

    session.examine("INBOX")?;

    Ok(session)
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

pub fn close<T: ReadWrite>(mut session: Session<T>) {
    session.logout().unwrap();
}