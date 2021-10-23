use crate::character::Item;
use ron;
use std::lazy::SyncLazy;

static ITEMS: SyncLazy<Vec<Item>> = SyncLazy::new(|| {
    ron::from_str::<Vec<Item>>(&String::from_utf8_lossy(include_bytes!("SRD/items.ron"))).unwrap()
});

pub fn get_items() -> Vec<Item> {
    ITEMS.to_vec()
}

pub fn get_item(name: &str) -> Option<Item> {
    ITEMS.iter().find(|e| e.name == name).cloned()
}
