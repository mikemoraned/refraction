use std::{net::TcpStream, io::{Read, Write}};
use imap::Session;
use native_tls::TlsStream;

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

pub fn open_tls_session(domain: &str, port: u16, username: &str, password: &str) 
    -> imap::error::Result<Session<TlsStream<TcpStream>>> {
    let tls = native_tls::TlsConnector::builder().build().unwrap();

    // we pass in the domain twice to check that the server's TLS
    // certificate is valid for the domain we're connecting to.
    let client = 
        imap::connect((domain, port), domain, &tls).unwrap();

    // the client we have here is unauthenticated.
    // to do anything useful with the e-mails, we need to log in
    let mut imap_session = client
        .login(username, password)
        .map_err(|e| e.0)?;

    // we want to fetch the first email in the INBOX mailbox
    // imap_session.select("INBOX")?;
    imap_session.examine("INBOX")?;

    Ok(imap_session)
}

pub fn close<T: Read + Write>(mut session: Session<T>) {
    session.logout().unwrap();
}