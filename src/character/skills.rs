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
        match self {
            Skill::Acrobatics => write!(f, "Acrobatics"),
            Skill::AnimalHandling => write!(f, "Animal Handling"),
            Skill::Arcana => write!(f, "Arcana"),
            Skill::Athletics => write!(f, "Athletics"),
            Skill::Deception => write!(f, "Deception"),
            Skill::History => write!(f, "History"),
            Skill::Insight => write!(f, "Insight"),
            Skill::Intimidation => write!(f, "Intimidation"),
            Skill::Investigation => write!(f, "Investigation"),
            Skill::Medicine => write!(f, "Medicine"),
            Skill::Nature => write!(f, "Nature"),
            Skill::Perception => write!(f, "Perception"),
            Skill::Performance => write!(f, "Performance"),
            Skill::Persuasion => write!(f, "Persuasion"),
            Skill::Religion => write!(f, "Religion"),
            Skill::SleightOfHand => write!(f, "Sleight of Hand"),
            Skill::Stealth => write!(f, "Stealth"),
            Skill::Survival => write!(f, "Survival"),
        }
    }
}

impl Skill {
    pub fn attr(&self) -> Ability {
        match self {
            Skill::Acrobatics => Ability::Dexterity,
            Skill::AnimalHandling => Ability::Wisdom,
            Skill::Arcana => Ability::Intelligence,
            Skill::Athletics => Ability::Strength,
            Skill::Deception => Ability::Charisma,
            Skill::History => Ability::Intelligence,
            Skill::Insight => Ability::Wisdom,
            Skill::Intimidation => Ability::Charisma,
            Skill::Investigation => Ability::Intelligence,
            Skill::Medicine => Ability::Wisdom,
            Skill::Nature => Ability::Intelligence,
            Skill::Perception => Ability::Wisdom,
            Skill::Performance => Ability::Charisma,
            Skill::Persuasion => Ability::Charisma,
            Skill::Religion => Ability::Intelligence,
            Skill::SleightOfHand => Ability::Dexterity,
            Skill::Stealth => Ability::Dexterity,
            Skill::Survival => Ability::Wisdom,
        }
    }
}
