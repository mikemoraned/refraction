use std::env;
use std::io::BufReader;
use atom_syndication::Feed;
use std::fs::File;
use std::io::Write;
use std::fs::OpenOptions;

use refraction::config;
use refraction::config::ConnectMethod;
use refraction::imap_session;
use refraction::imap_to_feed;
use refraction::feed;

fn main() -> Result<(), ()> {
    let config = config::from_path("refraction.toml");
    let mut total_changed_count = 0;

    for source in config.sources.unwrap() {
        let env_var = format!("{}_PASSWORD", source.id.to_ascii_uppercase());
        println!("Doing {:?}, using password in {}", source, env_var);

        let password = env::var(env_var).unwrap();

        let mut imap_session = match source.method {
            ConnectMethod::STARTTLS => 
                imap_session::open_starttls_session(&source.domain, source.port, &source.username, &password).unwrap(),
            ConnectMethod::TLS => 
                imap_session::open_tls_session(&source.domain, source.port, &source.username, &password).unwrap()
        };

        let all_feeds = config.feeds.clone().unwrap();
        let feeds_for_source : Vec<&config::FeedConfig>
            = all_feeds.iter().filter(|f| f.source == source.id).collect();
        for feed_config in feeds_for_source {
            println!("Doing {:?}", feed_config);

            let feed_file_path = format!("./dumped/{}.xml", feed_config.id);
            let (existing_feed, query) = match File::open(&feed_file_path) {
                Ok(input_file) => {
                    println!("Reading '{}'", feed_file_path);
                    let feed = Feed::read_from(BufReader::new(input_file)).unwrap();
                    let query = imap_to_feed::email_since_query(&feed_config.email, &feed.updated().date_naive());
                    (feed, query)
                },
                Err(error) => {
                    match error.kind() {
                        std::io::ErrorKind::NotFound => {
                            println!("File '{}' doesn't exist, so starting from empty Feed", feed_file_path);
                            let feed = Feed::default();
                            let query = imap_to_feed::email_query(&feed_config.email);
                            (feed, query)
                        },
                        _ => {
                            panic!("Unexpected error opening {}: {:?}", feed_file_path, error)
                        }
                    }
                }
            };
            println!("Using Query: {:?}", query);

            let mut new_feed = Feed::default();
            new_feed.set_title(feed_config.title.clone());

            let author = &feed_config.email;
            let fetch_entries = imap_to_feed::fetch_entries(&mut imap_session, &author, &query);
            if let Ok(mut possibly_new_entries) = fetch_entries {
                println!("possibly new entries: {}", possibly_new_entries.len());
                for entry in &possibly_new_entries {
                    println!("Title: '{}', Tag: '{}'", entry.title().to_string(), entry.id());
                }

                let mut all_entries = existing_feed.entries.clone();
                all_entries.append(&mut possibly_new_entries);
                let deduped_entries = feed::dedupe_entries_by_id(&all_entries);
                let changed_count = deduped_entries.len() - existing_feed.entries.len();
                println!("existing: {}, latest: {}, changed: {}", 
                    existing_feed.entries.len(),
                    deduped_entries.len(),
                    changed_count);

                if changed_count > 0 {

                    let latest_date = deduped_entries.iter().map(|e| e.updated).max().unwrap();
                    new_feed.set_entries(deduped_entries);
                    new_feed.set_updated(latest_date);

                    let mut output_file = OpenOptions::new()
                        .write(true)
                        .create(true)
                        .append(false)
                        .open(feed_file_path).unwrap();
                    output_file.write_all(new_feed.to_string().as_bytes()).unwrap();
                }
                else {
                    println!("Skipping {:?} as nothing changed", feed_config);
                }
                total_changed_count += changed_count;
            }
            else {
                println!("Got error, skipping: {:?}", fetch_entries);
            }   
        }

        imap_session::close(imap_session);
    }
    println!("Total entries changed: {}", total_changed_count);
    
    Ok(())
}
