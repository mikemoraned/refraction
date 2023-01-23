use std::{net::TcpStream, io::{Read, Write}};
use imap::{Session, types::{Fetch, ZeroCopy}, Error};

pub struct ImapSession<T: Read + Write> {
    internal: Session<T>
}

impl<T: Read + Write> ImapSession<T> {
    pub fn fetch<S1, S2>(&mut self, sequence_set : S1, query: S2) -> Result<ZeroCopy<Vec<Fetch>>, Error>
        where S1: AsRef<str>, S2: AsRef<str>
    {
        self.internal.fetch(sequence_set, query)
    }

    pub fn close(mut self) {
        self.internal.logout().unwrap();
    }
}

impl ImapSession<TcpStream> {
    pub fn open_session(domain: &str, port: u16, username: &str, password: &str) 
        -> imap::error::Result<ImapSession<TcpStream>> {
        let stream = TcpStream::connect((domain, port)).unwrap();
        let client = imap::Client::new(stream);

        // the client we have here is unauthenticated.
        // to do anything useful with the e-mails, we need to log in
        let mut internal = client
            .login(username, password)
            .map_err(|e| e.0)?;

        internal.examine("INBOX")?;

        Ok(ImapSession { internal })
    }
}