use std::collections::BTreeMap;

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
#[serde(default)]
pub struct Background {
    pub name: String,
    pub traits: BTreeMap<String, String>,
}

impl Default for Background {
    fn default() -> Self {
        Self {
            name: "Commoner".to_string(),
            traits: BTreeMap::new(),
        }
    }
}
