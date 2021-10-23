use std::collections::BTreeMap;

mod abilities;
mod background;
mod class;
mod feats;
mod items;
mod proficiency;
mod race;
mod skills;

pub use abilities::{Abilities, Ability};
pub use background::Background;
pub use class::{Class, Saves};
pub use feats::Feat;
pub use items::{EquipState, Item, ItemType};
pub use proficiency::Proficiency;
pub use race::Race;
pub use skills::{Skill, Skills};

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
    pub max_hp: u16,
    pub hp: i16,
    pub temp_hp: i16,
    pub background: Background,
    pub items: Vec<Item>,
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
            abilities: Abilities::new(),
            saves: Saves::default(),
            skills: Skills::default(),
            max_hp: 0,
            hp: 0,
            temp_hp: 0,
            background: Background::default(),
            items: Vec::new(),
        }
    }
}

impl Character {
    pub fn proficiency_bonus(&self) -> u8 {
        (self.level - 1) / 4 + 2
    }

    fn get_modifiers(&self, pattern: &str) -> Vec<(String, String)> {
        let mut modifiers = self.race.traits.clone();
        self.class
            .features
            .clone()
            .into_iter()
            .filter(|(k, _v)| *k <= self.level)
            .collect::<BTreeMap<u8, BTreeMap<String, String>>>()
            .values()
            .for_each(|e| modifiers.append(&mut e.clone()));

        modifiers
            .iter()
            .filter(|(k, _v)| k.starts_with(pattern))
            .map(|(k, v)| (k.clone(), v.clone()))
            .collect::<Vec<(String, String)>>()
    }

    pub fn ac(&self) -> u8 {
        let ac = (self.ability_mod(Ability::Dexterity) + 10) as u8;

        let armor = self
            .items
            .iter()
            .filter(|i| i.item_type == ItemType::Armor && i.equipped == EquipState::Equipped)
            .map(|i| {
                let dex_mod = self.ability_mod(Ability::Dexterity);
                match i.category.to_lowercase().as_str() {
                    "light" => (i.armor_class as i8 + dex_mod) as u8,
                    "medium" => (i.armor_class as i8 + i8::min(dex_mod, 2)) as u8,
                    "heavy" => i.armor_class,
                    _ => panic!("Unknown armor category: {}", i.category),
                }
            })
            .max();

        let shield = self
            .items
            .iter()
            .filter(|i| i.item_type == ItemType::Shield && i.equipped == EquipState::Equipped)
            .map(|i| i.armor_class)
            .max();

        armor.unwrap_or(ac) + shield.unwrap_or(0)
    }

    pub fn speed(&self) -> u8 {
        let mut speed = self.race.speed;

        let modifiers = self
            .get_modifiers(".abilities.")
            .iter()
            .map(|(k, v)| (k.clone(), v.parse::<i8>().unwrap()))
            .collect::<Vec<(String, i8)>>();

        for i in modifiers {
            speed += i.1 as u8;
        }

        speed
    }

    pub fn abilities(&self) -> Abilities {
        let mut abs = self.abilities + self.race.abilities;

        let modifiers = self
            .get_modifiers(".abilities.")
            .iter()
            .map(|(k, v)| (k.clone(), v.parse::<u8>().unwrap()))
            .collect::<Vec<(String, u8)>>();

        for i in modifiers {
            match i.0.as_str() {
                ".abilities.strength" => abs.strength += i.1,
                ".abilities.dexterity" => abs.dexterity += i.1,
                ".abilities.constitution" => abs.constitution += i.1,
                ".abilities.intelligence" => abs.intelligence += i.1,
                ".abilities.wisdom" => abs.wisdom += i.1,
                ".abilities.charisma" => abs.charisma += i.1,
                _ => panic!("Unknown ability: {}", i.0),
            }
        }

        abs
    }

    pub fn ability_score(&self, ability: Ability) -> u8 {
        match ability {
            Ability::Strength => self.abilities().strength,
            Ability::Dexterity => self.abilities().dexterity,
            Ability::Constitution => self.abilities().constitution,
            Ability::Intelligence => self.abilities().intelligence,
            Ability::Wisdom => self.abilities().wisdom,
            Ability::Charisma => self.abilities().charisma,
        }
    }

    pub fn ability_score_mut(&mut self, ability: Ability) -> &mut u8 {
        match ability {
            Ability::Strength => &mut self.abilities.strength,
            Ability::Dexterity => &mut self.abilities.dexterity,
            Ability::Constitution => &mut self.abilities.constitution,
            Ability::Intelligence => &mut self.abilities.intelligence,
            Ability::Wisdom => &mut self.abilities.wisdom,
            Ability::Charisma => &mut self.abilities.charisma,
        }
    }

    pub fn ability_mod(&self, ability: Ability) -> i8 {
        let x = self.ability_score(ability);
        let x = if x < 10 { x as i8 - 1 } else { x as i8 };

        (x - 10) / 2
    }

    pub fn skills(&self) -> Skills {
        let mut skl = self.skills.clone();

        let mut modifiers = self.race.traits.clone();
        modifiers.append(&mut self.background.traits.clone());
        self.class
            .features
            .clone()
            .into_iter()
            .filter(|(k, _v)| *k <= self.level)
            .collect::<BTreeMap<u8, BTreeMap<String, String>>>()
            .values()
            .for_each(|e| modifiers.append(&mut e.clone()));

        let modifiers = self.get_modifiers(".skills.");

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
                _ => panic!("Unknown skill: {}", i.0),
            }
        }

        skl
    }

    pub fn skill_level(&self, skill: Skill) -> Proficiency {
        match skill {
            Skill::Acrobatics => self.skills().acrobatics,
            Skill::AnimalHandling => self.skills().animal_handling,
            Skill::Arcana => self.skills().arcana,
            Skill::Athletics => self.skills().athletics,
            Skill::Deception => self.skills().deception,
            Skill::History => self.skills().history,
            Skill::Insight => self.skills().insight,
            Skill::Intimidation => self.skills().intimidation,
            Skill::Investigation => self.skills().investigation,
            Skill::Medicine => self.skills().medicine,
            Skill::Nature => self.skills().nature,
            Skill::Perception => self.skills().perception,
            Skill::Performance => self.skills().performance,
            Skill::Persuasion => self.skills().persuasion,
            Skill::Religion => self.skills().religion,
            Skill::SleightOfHand => self.skills().sleight_of_hand,
            Skill::Stealth => self.skills().stealth,
            Skill::Survival => self.skills().survival,
        }
    }

    pub fn skill_level_mut(&mut self, skill: Skill) -> &mut Proficiency {
        match skill {
            Skill::Acrobatics => &mut self.skills.acrobatics,
            Skill::AnimalHandling => &mut self.skills.animal_handling,
            Skill::Arcana => &mut self.skills.arcana,
            Skill::Athletics => &mut self.skills.athletics,
            Skill::Deception => &mut self.skills.deception,
            Skill::History => &mut self.skills.history,
            Skill::Insight => &mut self.skills.insight,
            Skill::Intimidation => &mut self.skills.intimidation,
            Skill::Investigation => &mut self.skills.investigation,
            Skill::Medicine => &mut self.skills.medicine,
            Skill::Nature => &mut self.skills.nature,
            Skill::Perception => &mut self.skills.perception,
            Skill::Performance => &mut self.skills.performance,
            Skill::Persuasion => &mut self.skills.persuasion,
            Skill::Religion => &mut self.skills.religion,
            Skill::SleightOfHand => &mut self.skills.sleight_of_hand,
            Skill::Stealth => &mut self.skills.stealth,
            Skill::Survival => &mut self.skills.survival,
        }
    }

    pub fn skill_mod(&self, skill: Skill) -> i8 {
        let x = self.ability_mod(skill.attr());

        x + (self.proficiency_bonus() * self.skill_level(skill).value()) as i8
    }
}

pub fn mod_str(val: i8) -> String {
    let s = if val < 0 { "-" } else { "+" };

    format!("{}{}", s, val.abs())
}
