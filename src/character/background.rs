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

impl Background {
    pub fn get_modifiers(
        &self,
        filter: impl Fn(&String, &String) -> bool,
    ) -> BTreeMap<String, String> {
        let mut modifiers = BTreeMap::new();

        self.traits
            .iter()
            .filter(|(k, v)| filter(k, v))
            .for_each(|(k, v)| {
                modifiers.insert(k.to_string(), v.to_string());
            });

        modifiers
    }
}
