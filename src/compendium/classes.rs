use crate::character::Class;
use ron;
use std::lazy::SyncLazy;

static CLASSES: SyncLazy<Vec<Class>> = SyncLazy::new(|| {
    let mut classes = ron::from_str::<Vec<Class>>(include_str!("SRD/classes.ron")).unwrap();
    classes.join(ron::from_str::<Vec<Class>>(include_str!("homebrew/classes.ron")).unwrap());

    classes
});

trait Joinable {
    fn join(&mut self, other: Self);
}

impl Joinable for Vec<Class> {
    fn join(&mut self, other: Self) {
        for o in other {
            match self.iter_mut().find(|s| s.name == o.name) {
                Some(s) => {
                    for i in o.subclasses.clone() {
                        s.subclasses.push(i);
                    }
                }
                None => {
                    self.push(o);
                }
            }
        }
    }
}

pub fn get_classes() -> Vec<Class> {
    CLASSES.to_vec()
}

pub fn get_class(name: &str) -> Option<Class> {
    CLASSES.iter().find(|e| e.name == name).cloned()
}
