use crate::character::{Ability, Proficiency};

#[derive(Clone, Copy, Debug, serde::Deserialize, serde::Serialize)]
pub enum Skill {
    Acrobatics,
    AnimalHandling,
    Arcana,
    Athletics,
    Deception,
    History,
    Insight,
    Intimidation,
    Investigation,
    Medicine,
    Nature,
    Perception,
    Performance,
    Persuasion,
    Religion,
    SleightOfHand,
    Stealth,
    Survival,
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

impl std::fmt::Display for Skill {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", match self {
            Self::Acrobatics => "Acrobatics",
            Self::AnimalHandling => "Animal Handling",
            Self::Arcana => "Arcana",
            Self::Athletics => "Athletics",
            Self::Deception => "Deception",
            Self::History => "History",
            Self::Insight => "Insight",
            Self::Intimidation => "Intimidation",
            Self::Investigation => "Investigation",
            Self::Medicine => "Medicine",
            Self::Nature => "Nature",
            Self::Perception => "Perception",
            Self::Performance => "Performance",
            Self::Persuasion => "Persuasion",
            Self::Religion => "Religion",
            Self::SleightOfHand => "Sleight of Hand",
            Self::Stealth => "Stealth",
            Self::Survival => "Survival",
        })
    }
}

impl Skill {
    pub fn attr(&self) -> Ability {
        match self {
            Self::Acrobatics => Ability::Dexterity,
            Self::AnimalHandling => Ability::Wisdom,
            Self::Arcana => Ability::Intelligence,
            Self::Athletics => Ability::Strength,
            Self::Deception => Ability::Charisma,
            Self::History => Ability::Intelligence,
            Self::Insight => Ability::Wisdom,
            Self::Intimidation => Ability::Charisma,
            Self::Investigation => Ability::Intelligence,
            Self::Medicine => Ability::Wisdom,
            Self::Nature => Ability::Intelligence,
            Self::Perception => Ability::Wisdom,
            Self::Performance => Ability::Charisma,
            Self::Persuasion => Ability::Charisma,
            Self::Religion => Ability::Intelligence,
            Self::SleightOfHand => Ability::Dexterity,
            Self::Stealth => Ability::Dexterity,
            Self::Survival => Ability::Wisdom,
        }
    }
}
