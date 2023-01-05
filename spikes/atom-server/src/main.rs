extern crate atom_syndication;

use atom_syndication::Feed;
use atom_syndication::Entry;

fn main() {
    let mut feed = Feed::default();
    let entries = vec![Entry::default()];
    feed.set_entries(entries);
    println!("feed: {:?}", feed);
}
