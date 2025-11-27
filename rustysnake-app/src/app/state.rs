use super::config::AppConfig;
use super::ui;
use crate::data::leaderboard::Leaderboard;
use crate::game::core::Game;
use crate::utils::themes::ThemeManager;
use eframe::{egui, App, Frame};

#[derive(PartialEq)]
pub enum AppScreen {
    MainMenu,
    Gameplay,
    Settings,
    Leaderboard,
    Paused,
}

pub struct SnakeApp {
    pub current_screen: AppScreen,
    pub game: Game,
    pub config: AppConfig,
    pub leaderboard: Leaderboard,
    pub theme_manager: ThemeManager,
    pub is_playing: bool,
}

impl Default for SnakeApp {
    fn default() -> Self {
        Self {
            current_screen: AppScreen::MainMenu,
            game: Game::new(),
            config: AppConfig::load().unwrap_or_default(),
            leaderboard: Leaderboard::load().unwrap_or_default(),
            theme_manager: ThemeManager::default(),
            is_playing: false,
        }
    }
}

impl App for SnakeApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut Frame) {
        // Apply the current theme
        self.theme_manager.current_theme = self.config.theme.clone();
        self.theme_manager.apply_theme(ctx);

        // Show the appropriate UI based on the current screen
        ui::update_ui(self, ctx);

        // If in gameplay, request a repaint to keep the game animating.
        // The duration can be tweaked for performance vs. responsiveness.
        if self.current_screen == AppScreen::Gameplay && !self.game.game_over {
            ctx.request_repaint_after(self.game.update_interval / 2);
        }
    }
}


impl SnakeApp {
    pub fn start_game(&mut self) {
        self.game = Game::new_with_config(&self.config);
        self.current_screen = AppScreen::Gameplay;
        self.is_playing = true;
    }
    
    pub fn pause_game(&mut self) {
        self.current_screen = AppScreen::Paused;
    }
    
    pub fn resume_game(&mut self) {
        self.current_screen = AppScreen::Gameplay;
    }
    
    pub fn game_over(&mut self) {
        if self.is_playing {
            self.leaderboard.add_score("Player", self.game.score);
            self.leaderboard.save().ok();
            self.is_playing = false;
        }
    }
}