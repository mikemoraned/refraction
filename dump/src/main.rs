use std::env;
use atom_syndication::Feed;
use std::io::Write;
use std::fs::OpenOptions;

mod config;
mod imap_session;
mod readability;
mod imap_to_feed;

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

        let entries = imap_to_feed::fetch_entries(&mut imap_session, &feed_config.email).unwrap();
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
