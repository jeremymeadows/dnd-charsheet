#[derive(Default, Clone, Debug, serde::Deserialize, serde::Serialize)]
#[serde(default)]
pub struct Feat {
    pub name: String,
    pub description: String,
    pub prerequisite: String,
}
