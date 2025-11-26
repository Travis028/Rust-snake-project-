pub mod main_menu;
pub mod game_screen;
pub mod settings_ui;

use eframe::egui;

use super::state::{SnakeApp, AppScreen};

pub fn update_ui(app: &mut SnakeApp, ctx: &egui::Context) {
    match app.current_screen {
        AppScreen::MainMenu => main_menu::show(app, ctx),
        AppScreen::Gameplay => game_screen::show(app, ctx),
        AppScreen::Settings => settings_ui::show(app, ctx),
        AppScreen::Leaderboard => show_leaderboard(app, ctx),
        AppScreen::Paused => show_pause_menu(app, ctx),
    }
}

fn show_leaderboard(app: &mut SnakeApp, ctx: &egui::Context) {
    egui::CentralPanel::default().show(ctx, |ui| {
        ui.heading("🏆 Leaderboard");
        
        for (i, score) in app.leaderboard.scores.iter().enumerate() {
            ui.label(format!("{}. {} - {}", i + 1, score.name, score.score));
        }
        
        if ui.button("Back to Menu").clicked() {
            app.current_screen = AppScreen::MainMenu;
        }
    });
}

fn show_pause_menu(app: &mut SnakeApp, ctx: &egui::Context) {
    egui::CentralPanel::default().show(ctx, |ui| {
        ui.heading("⏸️ Game Paused");
        
        if ui.button("▶️ Resume").clicked() {
            app.resume_game();
        }
        
        if ui.button("📊 Leaderboard").clicked() {
            app.current_screen = AppScreen::Leaderboard;
        }
        
        if ui.button("⚙️ Settings").clicked() {
            app.current_screen = AppScreen::Settings;
        }
        
        if ui.button("🚪 Main Menu").clicked() {
            app.current_screen = AppScreen::MainMenu;
            app.is_playing = false;
        }
    });
}
