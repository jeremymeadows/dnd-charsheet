#[derive(Clone, Copy, Debug, PartialEq, serde::Deserialize, serde::Serialize)]
pub enum ItemType {
    Currency,
    Armor,
    Shield,
    Weapon,
    Tool,
    Container,
    Other,
}

#[derive(Clone, Copy, Debug, PartialEq, serde::Deserialize, serde::Serialize)]
pub enum EquipState {
    None,
    Unequipped,
    Equipped,
}

impl Default for EquipState {
    fn default() -> Self {
        EquipState::None
    }
}

#[derive(Default, Clone, Debug, serde::Deserialize, serde::Serialize)]
#[serde(default)]
pub struct Item {
    pub name: String,
    pub description: String,
    pub item_type: ItemType,
    pub category: String,
    pub cost: String,
    pub weight: u16,
    pub armor_class: u8,
    pub equipped: EquipState,
    pub rarity: u32,
    pub properties: Vec<String>,
    pub required_proficiencies: Vec<String>,
}

impl Default for ItemType {
    fn default() -> Self {
        ItemType::Other
    }
}
