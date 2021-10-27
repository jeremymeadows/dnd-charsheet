use crate::character::abilities::Ability;
use std::collections::BTreeMap;

#[derive(Clone, Copy, Debug, PartialEq, serde::Deserialize, serde::Serialize)]
pub enum School {
    Abjuration,
    Conjuration,
    Divination,
    Enchantment,
    Evocation,
    Illusion,
    Necromancy,
    Transmutation,
    Other,
}

#[derive(Clone, Copy, Debug, PartialEq, serde::Deserialize, serde::Serialize)]
pub enum CastTime {
    Action,
    BonusAction,
    Reaction,
    Minutes(u8),
    Hours(u8),
}

#[derive(Clone, Copy, Debug, PartialEq, serde::Deserialize, serde::Serialize)]
pub enum Duration {
    Instantaneous,
    Rounds(u8),
    Minutes(u16),
    Hours(u16),
    Days(u16),
    Infinite,
}

#[derive(Clone, Copy, Debug, serde::Deserialize, serde::Serialize)]
pub enum DamageMode {
    None,
    Attack(DamageType),
    Save(Ability, DamageType),
}

#[derive(Clone, Copy, Debug, PartialEq, serde::Deserialize, serde::Serialize)]
pub enum DamageType {
    Acid,
    Cold,
    Fire,
    Force,
    Lightning,
    Necrotic,
    Poison,
    Psychic,
    Radiant,
    Thunder,
}

#[derive(Default, Clone, Debug, serde::Deserialize, serde::Serialize)]
#[serde(default)]
pub struct Spell {
    pub name: String,
    pub description: String,
    pub level: u8,
    pub school: School,
    pub ritual: bool,
    pub cast_time: CastTime,
    pub range: String,
    pub componens: Components,
    pub duration: Duration,
    pub concentration: bool,
    pub dice: Option<String>,
    pub damage: DamageMode,
    pub classes: Vec<String>,
    pub traits: BTreeMap<String, String>,
}

#[derive(Default, Clone, Debug, serde::Deserialize, serde::Serialize)]
#[serde(default)]
pub struct Components {
    pub verbal: bool,
    pub somatic: bool,
    pub material: Option<String>,
}

impl Default for School {
    fn default() -> Self {
        Self::Other
    }
}

impl Default for CastTime {
    fn default() -> Self {
        CastTime::Action
    }
}

impl Default for Duration {
    fn default() -> Self {
        Duration::Instantaneous
    }
}

impl Default for DamageMode {
    fn default() -> Self {
        DamageMode::None
    }
}
