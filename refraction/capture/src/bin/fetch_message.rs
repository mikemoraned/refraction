use capture::config;

fn main() {
    let imap_service = config::from_path("gmail.toml");
    println!("{:?}", imap_service);
}