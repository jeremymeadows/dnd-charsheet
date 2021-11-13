use std::collections::BTreeMap;

#[derive(Clone, Copy, Debug, PartialEq, serde::Deserialize, serde::Serialize)]
pub enum Value {
    Copper(u16),
    Silver(u16),
    Electrum(u16),
    Gold(u16),
    Platinum(u16),
}

#[derive(Clone, Copy, Debug, PartialEq, serde::Deserialize, serde::Serialize)]
pub enum ItemType {
    Currency,
    Armor(ArmorType),
    Shield,
    Weapon(WeaponType),
    Tool,
    Container,
    Other,
}

#[derive(Clone, Copy, Debug, PartialEq, serde::Deserialize, serde::Serialize)]
pub enum ArmorType {
    Light,
    Medium,
    Heavy,
}

#[derive(Clone, Copy, Debug, PartialEq, serde::Deserialize, serde::Serialize)]
pub enum WeaponType {
    SimpleMelee(DamageType),
    MartialMelee(DamageType),
    SimpleRanged(DamageType),
    MartialRanged(DamageType),
}

#[derive(Clone, Copy, Debug, PartialEq, serde::Deserialize, serde::Serialize)]
pub enum WeaponProperties {
    Ammunition,
    Finesse,
    Heavy,
    Light,
    Loading,
    Range,
    Reach,
    Thrown,
    TwoHanded,
    Versatile,
    Improvised,
    Silvered,
    Special,
}

#[derive(Clone, Copy, Debug, PartialEq, serde::Deserialize, serde::Serialize)]
pub enum DamageType {
    Bludgeoning,
    Piercing,
    Slashing,
    None,
}

#[derive(Clone, Copy, Debug, PartialEq, serde::Deserialize, serde::Serialize)]
pub enum EquipState {
    None,
    Unequipped,
    Equipped,
}

#[derive(Default, Clone, Debug, serde::Deserialize, serde::Serialize)]
#[serde(default)]
pub struct Item {
    pub name: String,
    pub description: String,
    pub item_type: ItemType,
    pub value: Value,
    pub weight: f32,
    pub armor_class: u8,
    pub damage_die: u8,
    pub range: String,
    pub equipped: EquipState,
    pub rarity: u32,
    pub properties: Vec<WeaponProperties>,
    pub required_proficiencies: Vec<String>,
    pub traits: BTreeMap<String, String>,
}

impl Default for Value {
    fn default() -> Self {
        Value::Copper(0)
    }
}

impl Default for ItemType {
    fn default() -> Self {
        Self::Other
    }
}

impl Default for EquipState {
    fn default() -> Self {
        EquipState::None
    }
}

impl std::fmt::Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Value::Copper(x) => write!(f, "{} cp", x),
            Value::Silver(x) => write!(f, "{} sp", x),
            Value::Electrum(x) => write!(f, "{} ep", x),
            Value::Gold(x) => write!(f, "{} gp", x),
            Value::Platinum(x) => write!(f, "{} pp", x),
        }
    }
}

impl std::fmt::Display for DamageType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Bludgeoning => write!(f, "Bludgeoning"),
            Self::Piercing => write!(f, "Piercing"),
            Self::Slashing => write!(f, "Slashing"),
            Self::None => write!(f, "None"),
        }
    }
}

impl EquipState {
    pub fn icon(&self) -> &str {
        match self {
            Self::Unequipped => "○",
            Self::Equipped => "⏺",
            Self::None => "✖",
        }
    }
}

impl std::ops::Not for EquipState {
    type Output = Self;

    fn not(self) -> Self {
        match self {
            Self::Unequipped => Self::Equipped,
            Self::Equipped => Self::Unequipped,
            Self::None => Self::None,
        }
    }
}
