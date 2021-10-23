use crate::character::Background;
use ron;
use std::lazy::SyncLazy;

static BACKGROUNDS: SyncLazy<Vec<Background>> = SyncLazy::new(|| {
    ron::from_str::<Vec<Background>>(&String::from_utf8_lossy(include_bytes!(
        "SRD/backgrounds.ron"
    )))
    .unwrap()
});

pub fn get_backgrounds() -> Vec<Background> {
    BACKGROUNDS.to_vec()
}

pub fn get_background(name: &str) -> Option<Background> {
    BACKGROUNDS.iter().find(|e| e.name == name).cloned()
}
