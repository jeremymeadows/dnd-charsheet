pub mod srd;

pub use crate::character::{Character, Race, Class, Background};

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
