use crate::character::Class;
use ron;
use std::lazy::SyncLazy;

static CLASSES: SyncLazy<Vec<Class>> = SyncLazy::new(|| {
    ron::from_str::<Vec<Class>>(&String::from_utf8_lossy(include_bytes!("SRD/classes.ron")))
        .unwrap()
});

pub fn get_classes() -> Vec<Class> {
    CLASSES.to_vec()
}

pub fn get_class(name: &str) -> Option<Class> {
    CLASSES.iter().find(|e| e.name == name).cloned()
}
