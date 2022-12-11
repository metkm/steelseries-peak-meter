use crate::models::{GameEvent, GameRegister, GameRegisterEvent, GameData};

const GAME_METADATA_ENDPOINT: &str = "game_metadata";
const GAME_EVENT_REGISTER_ENDPOINT: &str = "register_game_event";
const GAME_EVENT_ENDPOINT: &str = "game_event";


pub struct GameSense {
    game: String,
    address: String,
    client: reqwest::blocking::Client,
}
pub struct GameSenseBuilder {
    game: String,
    address: String,
    client: reqwest::blocking::Client,
}

impl GameSense {
    pub fn new(
        game: &str,
        game_display_name: &str,
        address: &str,
    ) -> reqwest::Result<GameSenseBuilder> {
        let client = reqwest::blocking::Client::new();
        let url = format!("http://{}/{}", &address, GAME_METADATA_ENDPOINT);

        let game_register_body = GameRegister {
            game,
            game_display_name,
        };

        client.post(url).json(&game_register_body).send()?;

        Ok(GameSenseBuilder {
            game: game.to_string(),
            address: address.to_string(),
            client,
        })
    }

    pub fn send_event(&self, event: &str, data: GameData) -> reqwest::Result<()> {
        let url = format!("http://{}/{}", self.address, GAME_EVENT_ENDPOINT);

        let body = GameEvent {
            game: &self.game,
            event,
            data
        };

        self.client.post(url).json(&body).send()?;

        Ok(())
    }
}

impl GameSenseBuilder {
    pub fn event(self, event_name: &str) -> reqwest::Result<Self> {
        let url = format!("http://{}/{}", &self.address, GAME_EVENT_REGISTER_ENDPOINT);

        let game_event_register_body = GameRegisterEvent {
            event: event_name,
            game: &self.game,
            min_value: 0,
            max_value: 100,
        };

        self.client
            .post(url)
            .json(&game_event_register_body)
            .send()?;

        Ok(self)
    }

    pub fn build(self) -> GameSense {
        GameSense {
            game: self.game,
            address: self.address,
            client: self.client,
        }
    }
}
