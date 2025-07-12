#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct Contact {
    pub name: String,
    pub phone: String,
    pub email: String,
}
