use serde::{Deserialize, Serialize};
use std::fs;

#[derive(Serialize, Deserialize, Clone)]
pub struct AppConfig {
    pub game_speed: u64,
    pub board_width: usize,
    pub board_height: usize,
    pub theme: String,
    pub sound_enabled: bool,
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            game_speed: 150,
            board_width: 20,
            board_height: 15,
            theme: "Dark".to_string(),
            sound_enabled: true,
        }
    }
}

impl AppConfig {
    pub fn load() -> Result<Self, Box<dyn std::error::Error>> {
        let config_str = fs::read_to_string("config/settings.ron")?;
        let config: AppConfig = ron::from_str(&config_str)?;
        Ok(config)
    }
    
    pub fn save(&self) -> Result<(), Box<dyn std::error::Error>> {
        let config_str = ron::to_string(self)?;
        fs::create_dir_all("config")?;
        fs::write("config/settings.ron", config_str)?;
        Ok(())
    }
}