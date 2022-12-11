use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct CoreProps {
    pub address: String,
    #[serde(rename="encryptedAddress")]
    pub encrypted_address: String,
    #[serde(rename="ggEncryptedAddress")]
    pub gg_encrypted_address: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GameEvent<'a> {
    pub game: &'a str,
    pub event: &'a str,
    pub data: GameData
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GameData {
    pub value: u32
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GameRegister<'a> {
    pub game: &'a str,
    pub game_display_name: &'a str
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GameRegisterEvent<'a> {
    pub game: &'a str,
    pub event: &'a str,
    pub min_value: i32,
    pub max_value: i32,
}
