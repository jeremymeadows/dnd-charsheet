use crate::character::Spell;
use ron;
use std::lazy::SyncLazy;

static SPELLS: SyncLazy<Vec<Spell>> =
    SyncLazy::new(|| ron::from_str::<Vec<Spell>>(include_str!("SRD/spells.ron")).unwrap());

pub fn get_spells() -> Vec<Spell> {
    SPELLS.to_vec()
}

pub fn get_spell(name: &str) -> Option<Spell> {
    SPELLS.iter().find(|e| e.name == name).cloned()
}
