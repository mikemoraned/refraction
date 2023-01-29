use atom_syndication::{Entry, Content};
use std::io::{Read, Write};
use imap::Session;
use imap::types::Fetch;
use imap_proto::types::BodyStructure::Text;
use quoted_printable::{decode, ParseMode};
use chrono::NaiveDate;

#[derive(Debug)]
pub struct Query(String);

impl Query {
    pub fn to_imap_query(&self) -> String {
        self.0.clone()
    }
}

pub fn email_query(email: &str) -> Query {
    Query(format!("FROM \"{}\"", email))
}

pub fn email_since_query(email: &str, date: &NaiveDate) -> Query {
    Query(format!("FROM \"{}\" SINCE {}", email, date.format("%d-%b-%Y")))
}

pub fn fetch_entries<T: Read + Write>(imap_session: &mut Session<T>, author: &str, query: &Query) -> imap::error::Result<Vec<Entry>> {
    let sequences = imap_session.search(query.to_imap_query())?;
    println!("sequences: {:?}", sequences);

    let sequence_set = sequences.into_iter().map(|s| s.to_string()).collect::<Vec<String>>().join(",");
    let messages = imap_session.fetch(
        sequence_set, 
        "(FLAGS INTERNALDATE RFC822.SIZE ENVELOPE BODYSTRUCTURE BODY.PEEK[TEXT])")?;
    let entries = messages.into_iter().flat_map(|message| {
        let mut entry = Entry::default();
    
        add_metadata_from_message(message, &author, &mut entry);
        
        if let Some(content) = get_message_content(message) {        
            entry.set_content(content);

            let readable_entry = crate::readability::readable_version(&entry);
            
            vec![entry, readable_entry]
        }
        else {
            println!("Skipping, as can't get content");
            vec![]
        }
    }).collect();
   
    Ok(entries)
}

fn get_message_content(message: &Fetch) -> Option<Content> {
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
                    return None
                }
            }
        },
        // TODO: Handle Multipart
        _ => { 
            println!("something else: {:?}", body_structure); 
            return None
        }
    };
    let mut content = Content::default();
    content.set_value(html.unwrap());
    content.set_content_type("text/html".to_string());
    Some(content)
}

fn add_metadata_from_message(message: &Fetch, author: &str, entry: &mut Entry) {
    use mail_parser::parsers::MessageStream;

    let internal_datetime = message.internal_date().unwrap();

    entry.set_updated(internal_datetime);

    let envelope = message.envelope().unwrap();

    let message_id = std::str::from_utf8(envelope.message_id.unwrap())
        .expect("was not valid utf-8")
        .to_string();
    println!("message id: '{}'", message_id);
    let tag = format!("tag:{},{}:{:x}", 
        author,
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
