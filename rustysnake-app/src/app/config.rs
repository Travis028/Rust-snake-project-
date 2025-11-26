// ... (paste the full content of config.rs from the original message) ...
#[derive(Serialize, Deserialize, Clone)]
pub struct AppConfig {
    // ... existing fields ...
    pub volume: f32,
    // ... rest of the struct ...
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            // ... existing fields ...
            volume: 0.5, // 50% volume by default
            // ... rest of the defaults ...
        }
    }
}
