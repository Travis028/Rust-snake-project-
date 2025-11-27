use eframe::egui;
use crate::app::state::SnakeApp;

pub fn show(app: &mut SnakeApp, ctx: &egui::Context) {
    egui::CentralPanel::default().show(ctx, |ui| {
        ui.heading("⚙️ Settings");
        
        ui.add_space(20.0);
        
        ui.label("Game Speed:");
        ui.add(egui::Slider::new(&mut app.config.game_speed, 50..=300).suffix("ms"));
        
        ui.add_space(10.0);
        
        ui.label("Board Size:");
        ui.horizontal(|ui| {
            ui.add(egui::Slider::new(&mut app.config.board_width, 10..=30).suffix("width"));
            ui.add(egui::Slider::new(&mut app.config.board_height, 10..=20).suffix("height"));
        });
        
        ui.add_space(10.0);
        
        ui.checkbox(&mut app.config.sound_enabled, "🔊 Sound Effects");
        
        ui.add_space(20.0);
        
        ui.horizontal(|ui| {
            if ui.button("💾 Save").clicked() {
                app.config.save().ok();
                ui.close_menu();
            }
            
            if ui.button("🚫 Cancel").clicked() {
                // Reload original config
                app.config = crate::app::config::AppConfig::load().unwrap_or_default();
                app.current_screen = crate::app::state::AppScreen::MainMenu;
            }
            
            if ui.button("🏠 Main Menu").clicked() {
                app.current_screen = crate::app::state::AppScreen::MainMenu;
            }
        });
    });
}