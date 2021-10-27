pub mod backgrounds;
pub mod classes;
pub mod feats;
pub mod items;
pub mod races;
pub mod spells;

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
