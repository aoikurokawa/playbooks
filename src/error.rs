#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Error {
    message: String,
    type: String,
}
