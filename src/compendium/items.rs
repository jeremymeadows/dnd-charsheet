use crate::character::{EquipState, Item, ItemType};
use ron;
use std::lazy::SyncLazy;

static ITEMS: SyncLazy<Vec<Item>> = SyncLazy::new(|| {
    let mut v = ron::from_str::<Vec<Item>>(include_str!("SRD/items.ron")).unwrap();
    v.iter_mut().for_each(|e| match e.item_type {
        ItemType::Armor(_) | ItemType::Shield | ItemType::Weapon(_) => {
            e.equipped = EquipState::Unequipped;
        }
        _ => {}
    });
    v
});

pub fn get_items() -> Vec<Item> {
    ITEMS.to_vec()
}

pub fn get_item(name: &str) -> Option<Item> {
    ITEMS.iter().find(|e| e.name == name).cloned()
}
