use std::env;
use atom_syndication::{Feed, Entry};
use std::io::Write;
use std::fs::OpenOptions;
use std::net::TcpStream;
use imap::Session;

fn main() -> Result<(), ()> {
    let args: Vec<String> = env::args().collect();

    let domain = "127.0.0.1";
    let port = 2143;
    let username = "mike@houseofmoran.com";
    let password = &args[1];
    let output_file_path = &args[2];
    let email = "pragmaticengineer@substack.com";

    let mut feed = Feed::default();
    feed.set_title(format!("Feed for '{}'", email));

    let mut imap_session = open_session(&domain,port, &username, &password).unwrap();

    let entries = fetch_entries(&mut imap_session, &email).unwrap();
    feed.set_entries(entries);

    imap_session.logout().unwrap();

    let mut output_file = OpenOptions::new()
        .write(true)
        .create(true)
        .append(false)
        .open(output_file_path).unwrap();
    output_file.write_all(feed.to_string().as_bytes()).unwrap();

    Ok(())
}

fn fetch_entries(imap_session: &mut Session<TcpStream>, email: &str) -> imap::error::Result<Vec<Entry>> {
    let sequences = imap_session.search(format!("FROM {}", email)).unwrap();
    println!("sequences: {:?}", sequences);

    let sequence_set = sequences.into_iter().map(|s| s.to_string()).collect::<Vec<String>>().join(",");
    let messages = imap_session.fetch(
        sequence_set, 
        "(FLAGS INTERNALDATE RFC822.SIZE ENVELOPE BODYSTRUCTURE BODY.PEEK[TEXT])").unwrap();
    for message in &messages {
        let envelope = message.envelope().unwrap();
        let subject = std::str::from_utf8(envelope.subject.unwrap())
            .expect("was not valid utf-8")
            .to_string();
        println!("subject: '{}'", subject);
    }
   
    Ok(vec![Entry::default()])
}

fn open_session(domain: &str, port: u16, username: &str, password: &str) -> imap::error::Result<Session<TcpStream>> {
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
