use crate::character::Feat;
use ron;
use std::lazy::SyncLazy;

static FEATS: SyncLazy<Vec<Feat>> = SyncLazy::new(|| {
    ron::from_str::<Vec<Feat>>(&String::from_utf8_lossy(include_bytes!("SRD/feats.ron"))).unwrap()
});

pub fn get_feats() -> Vec<Feat> {
    FEATS.to_vec()
}

pub fn get_feat(name: &str) -> Option<Feat> {
    FEATS.iter().find(|e| e.name == name).cloned()
}
