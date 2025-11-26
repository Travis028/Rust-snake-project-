// ... (paste the full content of core.rs from the original message) ...

impl Game {
    // ... existing methods ...

    pub fn update(&mut self, audio: Option<&AudioManager>) {
        if self.game_over {
            return;
        }

        if self.last_update.elapsed() >= self.update_interval {
            // ... existing update logic ...

            // When food is eaten
            if new_head.x == self.food.x && new_head.y == self.food.y {
                if let Some(audio) = audio {
                    audio.play_sound("assets/sounds/eat.wav");
                }
                // ... rest of food logic ...
            }

            // When game over
            if self.game_over {
                if let Some(audio) = audio {
                    audio.play_sound("assets/sounds/game_over.wav");
                }
            }
        }
    }
}
