pub mod character;
pub mod srd;

pub use character::{Character, Race, Class};

#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub enum Size { Fine, Diminutive, Tiny, Small, Medium, Large, Huge, Gargantuan, Colossal }

#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub enum Proficiency { None, Proficient, Expert }

#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub enum DamageType {
    Bludgeoning, Piercing, Slashing,
    Acid, Poison,
    Cold, Fire,
    Force, Psychic,
    Lightning, Thunder,
    Necrotic, Radiant,
}

#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub enum Conditions {
    Blinded,
    Charmed,
    Deafened,
    Frightened,
    Grappled,
    Incapacitated,
    Invisible,
    Paralyzed,
    Petrified,
    Poisoned,
    Prone,
    Restrained,
    Stunned,
    Surprised,
    Unconscious,
    Exhaustion,
}

impl Default for Size {
    fn default() -> Self {
        Self::Medium
    }
}

impl Default for Proficiency {
    fn default() -> Self {
        Self::None
    }
}
