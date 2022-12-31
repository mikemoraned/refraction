use std::collections::HashSet;

use atom_syndication::Entry;

pub fn dedupe_entries_by_id(entries: &Vec<Entry>) -> Vec<Entry> {
    let mut ids_seen = HashSet::new();
    let mut deduped_entries = Vec::new();
    for entry in entries {
        if !ids_seen.contains(entry.id()) {
            deduped_entries.push(entry.clone());
            ids_seen.insert(entry.id());
        }
    }
    deduped_entries
}