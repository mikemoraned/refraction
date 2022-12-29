extern crate imap;
extern crate native_tls;

use std::env;
use std::net::TcpStream;


fn main() -> Result<(), ()>{
    let args: Vec<String> = env::args().collect();
    let body = fetch_inbox_top(
        &args[1], 
        args[2].parse::<u16>().unwrap(), 
        &args[3], 
        &args[4]).unwrap();
    Ok(())
}

fn fetch_inbox_top(domain: &str, port: u16, username: &str, password: &str) -> imap::error::Result<()> {
    // let tls = native_tls::TlsConnector::builder().build().unwrap();

    // we pass in the domain twice to check that the server's TLS
    // certificate is valid for the domain we're connecting to.
    // let client = imap::connect((domain, port), domain, &tls).unwrap();

    let stream = TcpStream::connect((domain, port)).unwrap();
    let client = imap::Client::new(stream);

    // the client we have here is unauthenticated.
    // to do anything useful with the e-mails, we need to log in
    let mut imap_session = client
        .login(username, password)
        .map_err(|e| e.0)?;

    // we want to fetch the first email in the INBOX mailbox
    imap_session.select("INBOX")?;

    // fetch message number 1 in this mailbox, along with its RFC822 field.
    // RFC 822 dictates the format of the body of e-mails
    // let messages = imap_session.fetch("1", "RFC822")?;
    let messages = imap_session.fetch("1", "ALL")?;
    let message = if let Some(m) = messages.iter().next() {
        m
    } else {
        return Ok(());
    };

    let envelope = message.envelope().unwrap();
    println!("envelope: {:?}", envelope);
    let subject = std::str::from_utf8(envelope.subject.unwrap())
        .expect("was not valid utf-8")
        .to_string();
    println!("subject: '{}'", subject);

    // extract the message's body
    // let body = message.body().expect("message did not have a body!");
    // let body = std::str::from_utf8(body)
    //     .expect("message was not valid utf-8")
    //     .to_string();

    // be nice to the server and log out
    imap_session.logout()?;

    Ok(())
}