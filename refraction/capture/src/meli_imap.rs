use std::{env, collections::HashMap};

use melib::AccountSettings;
use melib::BackendEventConsumer;
use melib::MailBackend;
use melib::Result;
use melib::imap::ImapType;

use crate::config::{IMAPServer, ConnectMethod};

impl IMAPServer {
    pub fn open_meli_session(&self) -> Result<Box<dyn MailBackend>> {
        let env_var = format!("{}_PASSWORD", self.id.to_ascii_uppercase());
        println!("Doing {:?}, using password in {}", self, env_var);

        let password = env::var(env_var).unwrap();

        match self.method {
            ConnectMethod::STARTTLS => 
                unimplemented!(),
            ConnectMethod::TLS => 
                self.open_meli_tls_session(&password)
        }
    }

    pub fn open_meli_tls_session(&self, password: &str) -> Result<Box<dyn MailBackend>> {
        let extra : HashMap<String, String> = 
            [
                ("server_hostname".to_string(), self.domain.clone()),
                ("server_username".to_string(), self.username.clone()),
                ("server_password".to_string(), password.to_string()),
                ("server_port".to_string(), self.port.to_string()),
            ].iter().cloned().collect();
        let settings = AccountSettings {
            extra,
            ..Default::default()
        };
        let imap = ImapType::new(
            &settings,
            Box::new(|_| true),
            BackendEventConsumer::new(std::sync::Arc::new(|_, _| ())),
        )?;

        Ok(imap)
    }
}