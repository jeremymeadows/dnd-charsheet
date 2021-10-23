use crate::character::Proficiency;
use std::collections::BTreeMap;

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

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
#[serde(default)]
pub struct Class {
    pub name: String,
    pub hit_die: u8,
    pub saves: Saves,
    pub features: BTreeMap<u8, BTreeMap<String, String>>,
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
