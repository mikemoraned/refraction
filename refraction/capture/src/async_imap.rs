use std::env;
use anyhow::{bail, Result};
use futures::TryStreamExt;

use crate::config::{IMAPServer, ConnectMethod};

impl IMAPServer {
    pub async fn open_async_session(&self) -> Result<()> {
        let env_var = format!("{}_PASSWORD", self.id.to_ascii_uppercase());
        println!("Doing {:?}, using password in {}", self, env_var);

        let password = env::var(env_var).unwrap();

        match self.method {
            ConnectMethod::STARTTLS => 
                unimplemented!(),
            ConnectMethod::TLS => 
                self.open_async_tls_session(&password).await
        }
    }

    async fn open_async_tls_session(&self, password: &str) -> Result<()> {
        let imap_addr = (self.domain, self.port);
        let tcp_stream = TcpStream::connect(imap_addr).await?;
        let tls = async_native_tls::TlsConnector::new();
        let tls_stream = tls.connect(self.domain, tcp_stream).await?;

        Ok(())
    }
}