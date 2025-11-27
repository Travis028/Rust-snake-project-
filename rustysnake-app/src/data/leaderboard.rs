use serde::{Deserialize, Serialize};
use std::fs;

#[derive(Serialize, Deserialize, Clone)]
pub struct HighScore {
    pub name: String,
    pub score: u32,
}

#[derive(Default)]
pub struct Leaderboard {
    pub scores: Vec<HighScore>,
}

impl Leaderboard {
    pub fn load() -> Result<Self, Box<dyn std::error::Error>> {
        let data = fs::read_to_string("data/scores.json").unwrap_or_default();
        let scores: Vec<HighScore> = serde_json::from_str(&data).unwrap_or_default();
        Ok(Leaderboard { scores })
    }
    
    pub fn save(&self) -> Result<(), Box<dyn std::error::Error>> {
        let data = serde_json::to_string(&self.scores)?;
        fs::create_dir_all("data")?;
        fs::write("data/scores.json", data)?;
        Ok(())
    }
    
    pub fn add_score(&mut self, name: &str, score: u32) {
        self.scores.push(HighScore {
            name: name.to_string(),
            score,
        });
        
        // Sort descending by score and keep top 10
        self.scores.sort_by(|a, b| b.score.cmp(&a.score));
        if self.scores.len() > 10 {
            self.scores.truncate(10);
        }
    }
}