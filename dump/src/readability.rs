use atom_syndication::Entry;
use readable_readability::Readability;

pub fn readable_version(entry: &Entry) -> Entry {

    let mut readable_entry = entry.clone();
    readable_entry.set_id(format!("{}-readable", entry.id()));
    readable_entry.set_title(format!("{} [Readable]", entry.title().as_str()));

    let content = entry.content().unwrap();
    let html = content.value().unwrap();
    let (content_root, _) = Readability::new().parse(&html);
    let mut readable_bytes = vec![];
    content_root.serialize(&mut readable_bytes).unwrap();
    let readable_html = std::str::from_utf8(&readable_bytes)
                        .expect("text was not valid utf-8")
                        .to_string();
    let mut readable_content = entry.content().unwrap().clone();
    readable_content.set_value(readable_html);
    readable_entry.set_content(readable_content);

    readable_entry
}