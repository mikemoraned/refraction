
use std::{net::TcpStream, io::{Read, Write}, env};
use imap::Session;
use native_tls::TlsStream;

use crate::config::{IMAPServer, ConnectMethod};

impl IMAPServer {
    pub fn open_session(&self) -> imap::error::Result<Session<TlsStream<TcpStream>>> {
        let env_var = format!("{}_PASSWORD", self.id.to_ascii_uppercase());
        println!("Doing {:?}, using password in {}", self, env_var);

        let password = env::var(env_var).unwrap();

        match self.method {
            ConnectMethod::STARTTLS => 
                self.open_starttls_session(&password),
            ConnectMethod::TLS => 
                self.open_tls_session(&password)
        }
    }

    fn open_starttls_session(&self, password: &str) -> imap::error::Result<Session<TlsStream<TcpStream>>> {
        let tls = native_tls::TlsConnector::builder().build().unwrap();

        let client = 
            imap::connect_starttls((self.domain.clone(), self.port), self.domain.clone(), &tls).unwrap();

        let mut imap_session = client
            .login(self.username.clone(), password)
            .map_err(|e| e.0)?;

        imap_session.examine("INBOX")?;

        Ok(imap_session)
    }
    
    fn open_tls_session(&self, password: &str) -> imap::error::Result<Session<TlsStream<TcpStream>>> {
        let tls = native_tls::TlsConnector::builder().build().unwrap();

        let client = 
            imap::connect((self.domain.clone(), self.port), self.domain.clone(), &tls).unwrap();

        let mut imap_session = client
            .login(self.username.clone(), password)
            .map_err(|e| e.0)?;

        imap_session.examine("INBOX")?;

        Ok(imap_session)
    }

    pub fn close<T: Read + Write>(mut session: Session<T>) {
        session.logout().unwrap();
    }
}

