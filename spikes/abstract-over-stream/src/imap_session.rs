use std::net::TcpStream;
use imap::Session;

pub type ImapSession = Session<TcpStream>;

pub fn open_session(domain: &str, port: u16, username: &str, password: &str) 
    -> imap::error::Result<ImapSession> {
    let stream = TcpStream::connect((domain, port)).unwrap();
    let client = imap::Client::new(stream);

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

pub fn close_session(mut imap_session: ImapSession) {
    imap_session.logout().unwrap();
}