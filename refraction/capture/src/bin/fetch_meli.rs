use capture::config::{self};
use clap::Parser;
use melib::Result;

/// Fetch single message
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// message id to fetch
    #[arg(short, long)]
    message_id: u64,

    /// path to config file for imap server
    #[arg(short, long)]
    imap_server: String
}

#[tokio::main]
async fn main() -> Result<()> {
    let args = Args::parse();

    let imap_service = config::from_path(&args.imap_server);
    println!("{:?}", imap_service);

    let mut session = imap_service.open_meli_session().unwrap();
    println!("Opened session");

    let mailboxes = session.mailboxes().unwrap().await.unwrap();
    println!("Mailboxes: {:?}", mailboxes);
    // let sequence_set = format!("{}", args.message_id);
    // let messages = session.fetch(sequence_set, "RFC822").unwrap();
    // if let Some(m) = messages.iter().next() {
    //    println!("message: {:?}", m);

    //    let body = m.body().expect("message did not have a body!");
    //    let body = std::str::from_utf8(body)
    //    .expect("message was not valid utf-8")
    //    .to_string();

    //    println!("body: {}", body);
    // } 
    // IMAPServer::close(session);

    Ok(())
}