use std::collections::HashMap;
use super::{Proficiency, Size};

#[derive(Debug, serde::Deserialize, serde::Serialize)]
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
    pub background: String,
}

#[derive(Debug, serde::Deserialize, serde::Serialize)]
#[serde(default)]
pub struct Race {
    pub name: String,
    pub size: Size,
    pub speed: u8,
    pub abilities: Abilities,
    pub traits: HashMap<String, String>,
    pub modifiers: HashMap<String, String>,
}

#[derive(Debug, serde::Deserialize, serde::Serialize)]
#[serde(default)]
pub struct Class {
    pub name: String,
    pub hit_die: u8,
    pub saves: Saves,
    pub features: HashMap<u8, HashMap<String, String>>,
    pub modifiers: HashMap<String, String>,
}

#[derive(Debug, serde::Deserialize, serde::Serialize)]
#[serde(default)]
pub struct Abilities {
    pub strength: u8,
    pub dexterity: u8,
    pub constitution: u8,
    pub intelligence: u8,
    pub wisdom: u8,
    pub charisma: u8,
}

#[derive(Default, Debug, serde::Deserialize, serde::Serialize)]
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

#[derive(Default, Debug, serde::Deserialize, serde::Serialize)]
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
            background: "Acolyte".to_string(),
        }
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
            traits: HashMap::new(),
            modifiers: HashMap::new(),
        }
    }
}

impl Default for Class {
    fn default() -> Self {
        Self {
            name: "Fighter".to_string(),
            hit_die: 10,
            saves: Saves::default(),
            features: HashMap::new(),
            modifiers: HashMap::new(),
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
