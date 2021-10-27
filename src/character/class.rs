use crate::character::{Ability, Proficiency};
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
    pub spellcaster: Option<Ability>,
    pub hit_die: u8,
    pub saves: Saves,
    pub features: BTreeMap<u8, BTreeMap<String, String>>,
    pub subclasses: Vec<SubClass>,
}

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
#[serde(default)]
pub struct SubClass {
    pub name: String,
    pub description: String,
    pub features: BTreeMap<u8, BTreeMap<String, String>>,
}

impl Default for Class {
    fn default() -> Self {
        Self {
            name: "Fighter".to_string(),
            spellcaster: None,
            hit_die: 10,
            saves: Saves::default(),
            features: BTreeMap::new(),
            subclasses: Vec::new(),
        }
    }
}

impl Default for SubClass {
    fn default() -> Self {
        Self {
            name: "None".to_string(),
            description: String::new(),
            features: BTreeMap::new(),
        }
    }
}

impl std::ops::Add for Saves {
    type Output = Self;

    fn add(self, other: Saves) -> Self {
        Self {
            strength: self.strength + other.strength,
            dexterity: self.dexterity + other.dexterity,
            constitution: self.constitution + other.constitution,
            intelligence: self.intelligence + other.intelligence,
            wisdom: self.wisdom + other.wisdom,
            charisma: self.charisma + other.charisma,
            advantages: vec![self.advantages, other.advantages].concat(),
        }
    }
}

impl Class {
    pub fn get_modifiers(
        &self,
        level: u8,
        filter: impl Fn(&String, &String) -> bool,
    ) -> BTreeMap<String, String> {
        let mut modifiers = BTreeMap::new();

        self.features
            .clone()
            .into_iter()
            .filter(|(k, _v)| *k <= level)
            .collect::<BTreeMap<u8, BTreeMap<String, String>>>()
            .values()
            .for_each(|e| {
                e.iter().filter(|(k, v)| filter(k, v)).for_each(|(k, v)| {
                    modifiers.insert(k.to_string(), v.to_string());
                })
            });

        modifiers
    }

    pub fn get_subclass(&self, name: &str) -> Option<SubClass> {
        self.subclasses.iter().find(|e| e.name == name).cloned()
    }

    pub fn get_subclass_modifiers(
        &self,
        level: u8,
        name: &str,
        filter: impl Fn(&String, &String) -> bool,
    ) -> BTreeMap<String, String> {
        let mut modifiers = BTreeMap::new();

        self.get_subclass(name)
            .unwrap_or_default()
            .features
            .clone()
            .into_iter()
            .filter(|(k, _v)| *k <= level)
            .collect::<BTreeMap<u8, BTreeMap<String, String>>>()
            .values()
            .for_each(|e| {
                e.iter().filter(|(k, v)| filter(k, v)).for_each(|(k, v)| {
                    modifiers.insert(k.to_string(), v.to_string());
                })
            });

        modifiers
    }
}

impl SubClass {
    pub fn get_modifiers(
        &self,
        level: u8,
        filter: impl Fn(&String, &String) -> bool,
    ) -> BTreeMap<String, String> {
        let mut modifiers = BTreeMap::new();

        self.features
            .clone()
            .into_iter()
            .filter(|(k, _v)| *k <= level)
            .collect::<BTreeMap<u8, BTreeMap<String, String>>>()
            .values()
            .for_each(|e| {
                e.iter().filter(|(k, v)| filter(k, v)).for_each(|(k, v)| {
                    modifiers.insert(k.to_string(), v.to_string());
                })
            });

        modifiers
    }
}
