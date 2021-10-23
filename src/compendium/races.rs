use crate::character::Race;
use ron;
use std::lazy::SyncLazy;
use std::sync::Mutex;

static RACES: SyncLazy<Mutex<Vec<Race>>> = SyncLazy::new(|| {
    Mutex::new(
        ron::from_str::<Vec<Race>>(&String::from_utf8_lossy(include_bytes!("SRD/races.ron")))
            .unwrap(),
    )
});

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
