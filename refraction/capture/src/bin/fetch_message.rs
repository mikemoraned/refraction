use capture::config::{self, IMAPServer};

fn main() {
    let imap_service = config::from_path("gmail.toml");
    println!("{:?}", imap_service);

    let session = imap_service.open_session().unwrap();
    println!("Opened session");
    IMAPServer::close(session);
}