use std::env;
use atom_syndication::Feed;
use std::io::Write;
use std::fs::OpenOptions;
use std::net::TcpStream;


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

    let mut output_file = OpenOptions::new()
        .write(true)
        .create(true)
        .append(false)
        .open(output_file_path).unwrap();
    output_file.write_all(feed.to_string().as_bytes()).unwrap();

    let mut imap_session = open_session(&domain,port, &username, &password).unwrap();

    

    imap_session.logout().unwrap();

    Ok(())
}

fn open_session(domain: &str, port: u16, username: &str, password: &str) -> imap::error::Result<imap::Session<TcpStream>> {
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
