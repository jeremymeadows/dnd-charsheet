use std::collections::BTreeMap;

#[derive(Clone, Copy, Debug, serde::Deserialize, serde::Serialize)]
pub enum Size { Fine, Diminutive, Tiny, Small, Medium, Large, Huge, Gargantuan, Colossal }

#[derive(Clone, Copy, Debug, serde::Deserialize, serde::Serialize)]
pub enum Proficiency { None, Proficient, Expert }

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
#[serde(default)]
pub struct Character {
    pub name: String,
    pub gender: String,
    pub race: Race,
    pub class: Class,
    pub level: u8,
    pub alignment: String,
    pub abilities: Abilities,
    pub saves: Saves,
    pub skills: Skills,
    pub hp: i16,
    pub temp_hp: i16,
    pub background: Background,
}

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
#[serde(default)]
pub struct Race {
    pub name: String,
    pub size: Size,
    pub speed: u8,
    pub abilities: Abilities,
    pub traits: BTreeMap<String, String>,
}

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
#[serde(default)]
pub struct Class {
    pub name: String,
    pub hit_die: u8,
    pub saves: Saves,
    pub features: BTreeMap<u8, BTreeMap<String, String>>,
}

#[derive(Clone, Copy, Debug, serde::Deserialize, serde::Serialize)]
#[serde(default)]
pub struct Abilities {
    pub strength: u8,
    pub dexterity: u8,
    pub constitution: u8,
    pub intelligence: u8,
    pub wisdom: u8,
    pub charisma: u8,
}

#[derive(Default, Clone, Debug, serde::Deserialize, serde::Serialize)]
#[serde(default)]
pub struct Saves {
    pub strength: Proficiency,
    pub dexterity: Proficiency,
    pub constitution: Proficiency,
    pub intelligence: Proficiency,
    pub wisdom: Proficiency,
    pub charisma: Proficiency,
    pub advantages: Vec<String>,
}

#[derive(Default, Clone, Debug, serde::Deserialize, serde::Serialize)]
#[serde(default)]
pub struct Skills {
    pub acrobatics: Proficiency,
    pub animal_handling: Proficiency,
    pub arcana: Proficiency,
    pub athletics: Proficiency,
    pub deception: Proficiency,
    pub history: Proficiency,
    pub insight: Proficiency,
    pub intimidation: Proficiency,
    pub investigation: Proficiency,
    pub medicine: Proficiency,
    pub nature: Proficiency,
    pub perception: Proficiency,
    pub performance: Proficiency,
    pub persuasion: Proficiency,
    pub religion: Proficiency,
    pub sleight_of_hand: Proficiency,
    pub stealth: Proficiency,
    pub survival: Proficiency,
}

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
#[serde(default)]
pub struct Background {
    pub name: String,
    pub traits: BTreeMap<String, String>,
}

impl Default for Character {
    fn default() -> Self {
        Self {
            name: "Beowulf".to_string(),
            gender: "Male".to_string(),
            race: Race::default(),
            class: Class::default(),
            level: 1,
            alignment: "Neutral".to_string(),
            abilities: Abilities::default(),
            saves: Saves::default(),
            skills: Skills::default(),
            hp: 8,
            temp_hp: 0,
            background: Background::default(),
        }
    }
}

impl Default for Size {
    fn default() -> Self {
        Self::Medium
    }
}

impl Default for Proficiency {
    fn default() -> Self {
        Self::None
    }
}

impl Default for Race {
    fn default() -> Self {
        Self {
            name: "Human".to_string(),
            size: Size::Medium,
            speed: 30,
            abilities: Abilities {
                strength: 1,
                dexterity: 1,
                constitution: 1,
                intelligence: 1,
                wisdom: 1,
                charisma: 1,
            },
            traits: BTreeMap::new(),
        }
    }
}

impl Default for Class {
    fn default() -> Self {
        Self {
            name: "Fighter".to_string(),
            hit_die: 10,
            saves: Saves::default(),
            features: BTreeMap::new(),
        }
    }
}

impl Default for Abilities {
    fn default() -> Self {
        Self {
            strength: 10,
            dexterity: 10,
            constitution: 10,
            intelligence: 10,
            wisdom: 10,
            charisma: 10,
        }
    }
}

impl Default for Background {
    fn default() -> Self {
        Self {
            name: "Commoner".to_string(),
            traits: BTreeMap::new(),
        }
    }
}


impl std::ops::Add for Proficiency {
    type Output = Self;

    fn add(self, other: Proficiency) -> Self {
        use Proficiency::*;

        match self {
            None => other,
            Proficient => match other {
                None | Proficient => Proficient,
                Expert => Expert,
            }
            Expert => Expert,
        }
    }
}

impl std::ops::AddAssign for Proficiency {
    fn add_assign(&mut self, other: Proficiency) {
        *self = *self + other;
    }
}

impl std::ops::Add for Abilities {
    type Output = Self;

    fn add(self, other: Abilities) -> Self {
        Self {
            strength: self.strength + other.strength,
            dexterity: self.dexterity + other.dexterity,
            constitution: self.constitution + other.constitution,
            intelligence: self.intelligence + other.intelligence,
            wisdom: self.wisdom + other.wisdom,
            charisma: self.charisma + other.charisma,
        }
    }
}

impl Character {
    pub fn abilities(&self) -> Abilities {
        let mut abs = self.abilities + self.race.abilities;

        fn parse(s: &str) -> u8 {
            s.parse().unwrap()
        }

        let mut modifiers = self.race.traits.clone();
        self.class.features.clone().into_iter()
            .filter(|(k, _v)| *k <= self.level)
            .collect::<BTreeMap<u8, BTreeMap<String, String>>>()
            .values()
            .for_each(|e| modifiers.append(&mut e.clone()));

        let modifiers = modifiers.iter()
            .filter(|(k, _v)| k.starts_with(".abilities."))
            .map(|(k, v)| (k.clone(), parse(v)))
            .collect::<Vec<(String, u8)>>();

        for i in modifiers {
            match i.0.as_str() {
                ".abilities.strength" => abs.strength += i.1,
                ".abilities.dexterity" => abs.dexterity += i.1,
                ".abilities.constitution" => abs.constitution += i.1,
                ".abilities.intelligence" => abs.intelligence += i.1,
                ".abilities.wisdom" => abs.wisdom += i.1,
                ".abilities.charisma" => abs.charisma += i.1,
                _ => (),
            }
        }

        abs
    }

    pub fn skills(&self) -> Skills {
        let mut skl = self.skills.clone();

        let mut modifiers = self.race.traits.clone();
        modifiers.append(&mut self.background.traits.clone());
        self.class.features.clone().into_iter()
            .filter(|(k, _v)| *k <= self.level)
            .collect::<BTreeMap<u8, BTreeMap<String, String>>>()
            .values()
            .for_each(|e| modifiers.append(&mut e.clone()));

        let modifiers = modifiers.iter()
            .filter(|(k, _v)| k.starts_with(".skills."))
            .map(|(k, v)| (k.clone(), v.clone()))
            .collect::<Vec<(String, String)>>();

        for i in modifiers {
            let lvl = ron::from_str::<Proficiency>(&i.1).unwrap();

            match i.0.as_str() {
                ".skills.acrobatics" => skl.acrobatics += lvl,
                ".skills.animal_handling" => skl.animal_handling += lvl,
                ".skills.arcana" => skl.arcana += lvl,
                ".skills.athletics" => skl.athletics += lvl,
                ".skills.deception" => skl.deception += lvl,
                ".skills.history" => skl.history += lvl,
                ".skills.insight" => skl.insight += lvl,
                ".skills.intimidation" => skl.intimidation += lvl,
                ".skills.investigation" => skl.investigation += lvl,
                ".skills.medicine" => skl.medicine += lvl,
                ".skills.nature" => skl.nature += lvl,
                ".skills.perception" => skl.perception += lvl,
                ".skills.performance" => skl.performance += lvl,
                ".skills.persuasion" => skl.persuasion += lvl,
                ".skills.religion" => skl.religion += lvl,
                ".skills.sleight_of_hand" => skl.sleight_of_hand += lvl,
                ".skills.stealth" => skl.stealth += lvl,
                ".skills.survival" => skl.survival += lvl,
                _ => (),
            }
        }

        skl
    }

    pub fn calc_mod(val: u8) -> i8 {
        let val = if val < 10 {
            val as i8 - 1
        } else {
            val as i8
        };

        (val - 10) / 2
    }

    pub fn proficiency_bonus(&self) -> u8 {
        (self.level - 1) / 4 + 2
    }
}
