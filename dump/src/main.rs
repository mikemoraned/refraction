use std::env;
use atom_syndication::{Feed, Entry, Content};
use std::io::Write;
use std::fs::OpenOptions;
use std::net::TcpStream;
use imap::Session;
use imap::types::Fetch;
use imap_proto::types::BodyStructure::Text;
use quoted_printable::{decode, ParseMode};

mod config;
mod imap_session;
mod readability;

fn main() -> Result<(), ()> {
    let config = config::from_path("refraction.toml");

    let password = env::var("IMAP_PASSWORD").unwrap();

    let mut imap_session = 
        imap_session::open_session(&config.imap.domain, config.imap.port, &config.imap.username, &password).unwrap();

    for feed_config in config.feeds.unwrap() {
        println!("Doing {:?}", feed_config);

        let output_file_path = format!("./dumped/{}.xml", feed_config.id);

        let mut feed = Feed::default();
        feed.set_title(feed_config.title);

        let entries = fetch_entries(&mut imap_session, &feed_config.email).unwrap();
        let latest_date = entries.iter().map(|e| e.updated).max().unwrap();
        feed.set_entries(entries);
        feed.set_updated(latest_date);

        let mut output_file = OpenOptions::new()
            .write(true)
            .create(true)
            .append(false)
            .open(output_file_path).unwrap();
        output_file.write_all(feed.to_string().as_bytes()).unwrap();
    }

    imap_session::close_session(imap_session);

    Ok(())
}

fn fetch_entries(imap_session: &mut Session<TcpStream>, email: &str) -> imap::error::Result<Vec<Entry>> {
    let sequences = imap_session.search(format!("FROM {}", email)).unwrap();
    println!("sequences: {:?}", sequences);

    let sequence_set = sequences.into_iter().map(|s| s.to_string()).collect::<Vec<String>>().join(",");
    let messages = imap_session.fetch(
        sequence_set, 
        "(FLAGS INTERNALDATE RFC822.SIZE ENVELOPE BODYSTRUCTURE BODY.PEEK[TEXT])").unwrap();
    let entries = messages.into_iter().flat_map(|message| {
        let mut entry = Entry::default();
    
        add_metadata_from_message(message, &email, &mut entry);
        
        let content = get_message_content(message);
        entry.set_content(content);

        let readable_entry = readability::readable_version(&entry);

        vec![entry, readable_entry]
    }).collect();
   
    Ok(entries)
}

fn get_message_content(message: &Fetch) -> Content {
    let body_structure = message.bodystructure().unwrap();
    let html = match body_structure {
        t @ Text { .. } => { 
            println!("Text: {:?}", t); 
            match message.text() {
                Some(quoted_printable_bytes) => { 
                    let bytes = decode(
                        quoted_printable_bytes, ParseMode::Robust)
                        .expect("was not valid quoted_printable");
                    let text = std::str::from_utf8(&bytes)
                        .expect("text was not valid utf-8")
                        .to_string();
                    // println!("text: {}", text);
                    Some(text)
                },
                None => { 
                    println!("Missing text");
                    None
                }
            }
        },
        _ => { 
            println!("something else"); 
            None
        }
    };
    let mut content = Content::default();
    content.set_value(html.unwrap());
    content.set_content_type("text/html".to_string());
    content
}

fn add_metadata_from_message(message: &Fetch, email: &str, entry: &mut Entry) {
    use mail_parser::parsers::MessageStream;

    let internal_datetime = message.internal_date().unwrap();

    entry.set_updated(internal_datetime);

    let envelope = message.envelope().unwrap();

    let message_id = std::str::from_utf8(envelope.message_id.unwrap())
        .expect("was not valid utf-8")
        .to_string();
    println!("message id: '{}'", message_id);
    let tag = format!("tag:{},{}:{:x}", 
        email,
        internal_datetime.date_naive().format("%Y-%m-%d"),
        md5::compute(envelope.message_id.unwrap()));
    println!("tag '{}'", tag);
    entry.set_id(tag);
    
    let subject = std::str::from_utf8(envelope.subject.unwrap())
        .expect("was not valid utf-8")
        .to_string();
    println!("subject: '{}'", subject);
    if subject.starts_with('=') {
        let envelope_bytes = envelope.subject.unwrap();
        let missing_first_char = &envelope_bytes[1..];
        let subject = MessageStream::new(missing_first_char).decode_rfc2047().unwrap();
        println!("decoded subject: '{}'", subject);
        entry.set_title(subject);
    } else {
        entry.set_title(subject);
    }
}


