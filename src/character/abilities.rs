#[derive(Clone, Copy, Debug, serde::Deserialize, serde::Serialize)]
pub enum Ability {
    Strength,
    Dexterity,
    Constitution,
    Intelligence,
    Wisdom,
    Charisma,
}

#[derive(Default, Clone, Copy, Debug, serde::Deserialize, serde::Serialize)]
#[serde(default)]
pub struct Abilities {
    pub strength: u8,
    pub dexterity: u8,
    pub constitution: u8,
    pub intelligence: u8,
    pub wisdom: u8,
    pub charisma: u8,
}

impl Abilities {
    pub fn new() -> Self {
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

impl std::fmt::Display for Ability {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Ability::Strength => write!(f, "Strength"),
            Ability::Dexterity => write!(f, "Dexterity"),
            Ability::Constitution => write!(f, "Constitution"),
            Ability::Intelligence => write!(f, "Intelligence"),
            Ability::Wisdom => write!(f, "Wisdom"),
            Ability::Charisma => write!(f, "Charisma"),
        }
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

impl Ability {
    pub fn to_string_short(&self) -> String {
        match self {
            Ability::Strength => "STR".to_string(),
            Ability::Dexterity => "DEX".to_string(),
            Ability::Constitution => "CON".to_string(),
            Ability::Intelligence => "INT".to_string(),
            Ability::Wisdom => "WIS".to_string(),
            Ability::Charisma => "CHA".to_string(),
        }
    }
}
