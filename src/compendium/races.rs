use crate::character::Race;
use ron;
use std::lazy::SyncLazy;
use std::sync::Mutex;

static RACES: SyncLazy<Mutex<Vec<Race>>> = SyncLazy::new(|| {
    let mut races = ron::from_str::<Vec<Race>>(include_str!("SRD/races.ron")).unwrap();
    races.join(ron::from_str::<Vec<Race>>(include_str!("homebrew/races.ron")).unwrap());

    Mutex::new(races)
});

trait Joinable {
    fn join(&mut self, other: Self);
}

impl Joinable for Vec<Race> {
    fn join(&mut self, other: Self) {
        for o in other {
            match self.iter_mut().find(|s| s.name == o.name) {
                Some(s) => {
                    if s.subraces.is_some() && o.subraces.is_some() {
                        for i in o.subraces.clone().unwrap() {
                            s.subraces.as_mut().unwrap().push(i);
                        }
                    }
                }
                None => {
                    self.push(o);
                }
            }
        }
    }
}

pub fn get_races() -> Vec<Race> {
    RACES.lock().unwrap().to_vec()
}

pub fn get_race(name: &str) -> Option<Race> {
    let races = RACES.lock().unwrap();
    let race = match races.iter().find(|e| e.name == name) {
        Some(race) => race.clone(),
        None => {
            // get the race which contains the subrace the user selected
            let mut race = races
                .iter()
                .filter(|e| e.subraces.is_some())
                .find(|e| e.subraces.as_ref().unwrap().iter().any(|e| e.name == name))
                .cloned()
                .unwrap();

            // get the subrace the user selected
            let mut subrace = race
                .subraces
                .as_ref()
                .unwrap()
                .iter()
                .find(|e| e.name == name)
                .cloned()
                .unwrap();

            race.traits.append(&mut subrace.traits);

            Race {
                name: subrace.name,
                size: race.size,
                speed: race.speed,
                abilities: race.abilities + subrace.abilities,
                traits: race.traits,
                subraces: None,
            }
        }
    };

    Some(race)
}
