use crate::character::Abilities;
use std::collections::BTreeMap;

#[derive(Clone, Copy, Debug, serde::Deserialize, serde::Serialize)]
pub enum Size {
    Fine,
    Diminutive,
    Tiny,
    Small,
    Medium,
    Large,
    Huge,
    Gargantuan,
    Colossal,
}

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
#[serde(default)]
pub struct Race {
    pub name: String,
    pub size: Size,
    pub speed: u8,
    pub abilities: Abilities,
    pub traits: BTreeMap<String, String>,
    pub subraces: Option<Vec<SubRace>>,
}

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
#[serde(default)]
pub struct SubRace {
    pub name: String,
    pub abilities: Abilities,
    pub traits: BTreeMap<String, String>,
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
            subraces: None,
        }
    }
}

impl Default for SubRace {
    fn default() -> Self {
        Self {
            name: "Variant".to_string(),
            abilities: Abilities {
                strength: 0,
                dexterity: 0,
                constitution: 0,
                intelligence: 0,
                wisdom: 0,
                charisma: 0,
            },
            traits: BTreeMap::new(),
        }
    }
}
