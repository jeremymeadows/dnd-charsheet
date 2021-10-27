use crate::character::Background;
use ron;
use std::lazy::SyncLazy;

static BACKGROUNDS: SyncLazy<Vec<Background>> = SyncLazy::new(|| {
    [
        ron::from_str::<Vec<Background>>(include_str!("SRD/backgrounds.ron")).unwrap(),
        ron::from_str::<Vec<Background>>(include_str!("homebrew/backgrounds.ron")).unwrap(),
    ]
    .concat()
});

pub fn get_backgrounds() -> Vec<Background> {
    BACKGROUNDS.to_vec()
}

pub fn get_background(name: &str) -> Option<Background> {
    BACKGROUNDS.iter().find(|e| e.name == name).cloned()
}
