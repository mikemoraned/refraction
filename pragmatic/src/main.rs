use std::env;
use atom_syndication::Feed;
use std::io::Write;
use std::fs::OpenOptions;


fn main() {
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
}
