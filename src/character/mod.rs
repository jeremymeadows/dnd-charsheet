mod abilities;
mod background;
mod class;
mod feats;
mod item;
mod proficiency;
mod race;
mod skills;
mod spell;

pub use abilities::{Abilities, Ability};
pub use background::Background;
pub use class::{Class, Saves, SubClass};
pub use feats::Feat;
pub use item::{ArmorType, EquipState, Item, ItemType};
pub use proficiency::Proficiency;
pub use race::Race;
pub use skills::{Skill, Skills};
pub use spell::Spell;

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
#[serde(default)]
pub struct Character {
    pub name: String,
    pub gender: String,
    pub race: Race,
    pub class: Class,
    pub spec: SubClass,
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
            spec: SubClass::default(),
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
        let mut modifiers = self.race.get_modifiers(|k, _v| k.starts_with(pattern));
        modifiers.append(
            &mut self
                .background
                .get_modifiers(|k, _v| k.starts_with(pattern)),
        );
        modifiers.append(
            &mut self
                .class
                .get_modifiers(self.level, |k, _v| k.starts_with(pattern)),
        );
        modifiers.append(
            &mut self
                .spec
                .get_modifiers(self.level, |k, _v| k.starts_with(pattern)),
        );

        modifiers
            .iter()
            .map(|(k, v)| (k.clone(), v.clone()))
            .collect::<Vec<(String, String)>>()
    }

    pub fn ac(&self) -> u8 {
        let ac = (self.ability_mod(Ability::Dexterity) + 10) as u8;

        let armor = self
            .items
            .iter()
            .filter(|i| {
                matches!(i.item_type, ItemType::Armor(_)) && i.equipped == EquipState::Equipped
            })
            .map(|i| {
                if let ItemType::Armor(a) = i.item_type {
                    let dex_mod = self.ability_mod(Ability::Dexterity);
                    match a {
                        ArmorType::Light => (i.armor_class as i8 + dex_mod) as u8,
                        ArmorType::Medium => (i.armor_class as i8 + i8::min(dex_mod, 2)) as u8,
                        ArmorType::Heavy => i.armor_class,
                    }
                } else {
                    0
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

        for i in self.get_modifiers(".abilities.") {
            let val = i.1.parse::<u8>().unwrap();

            match i.0.as_str() {
                ".abilities.strength" => abs.strength += val,
                ".abilities.dexterity" => abs.dexterity += val,
                ".abilities.constitution" => abs.constitution += val,
                ".abilities.intelligence" => abs.intelligence += val,
                ".abilities.wisdom" => abs.wisdom += val,
                ".abilities.charisma" => abs.charisma += val,
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

    pub fn saves(&self) -> Saves {
        let mut svs = self.saves.clone() + self.class.saves.clone();

        for i in self.get_modifiers(".saves.ability.") {
            let lvl = ron::from_str::<Proficiency>(&i.1).unwrap();

            match i.0.as_str() {
                ".saves.ability.strength" => svs.strength += lvl,
                ".saves.ability.dexterity" => svs.dexterity += lvl,
                ".saves.ability.constitution" => svs.constitution += lvl,
                ".saves.ability.intelligence" => svs.intelligence += lvl,
                ".saves.ability.wisdom" => svs.wisdom += lvl,
                ".saves.ability.charisma" => svs.charisma += lvl,
                _ => panic!("Unknown ability: {}", i.0),
            }
        }

        svs
    }

    pub fn save(&self, ability: Ability) -> Proficiency {
        match ability {
            Ability::Strength => self.saves().strength,
            Ability::Dexterity => self.saves().dexterity,
            Ability::Constitution => self.saves().constitution,
            Ability::Intelligence => self.saves().intelligence,
            Ability::Wisdom => self.saves().wisdom,
            Ability::Charisma => self.saves().charisma,
        }
    }

    pub fn save_mod(&self, ability: Ability) -> i8 {
        let x = self.ability_mod(ability);

        x + (self.proficiency_bonus() * self.save(ability).value()) as i8
    }

    pub fn skills(&self) -> Skills {
        let mut skl = self.skills.clone();

        for i in self.get_modifiers(".skills.") {
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

    pub fn assign_modifiers(&mut self) {
        if let Some(s) = self.get_modifiers(".spellcasting").first() {
            self.class.spellcaster = Some(ron::from_str::<Ability>(&s.1).unwrap());
        }
    }
}

pub fn mod_str(val: i8) -> String {
    let s = if val < 0 { "-" } else { "+" };

    format!("{}{}", s, val.abs())
}
