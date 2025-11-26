use eframe::egui;
use crate::app::state::SnakeApp;

pub fn show(app: &mut SnakeApp, ctx: &egui::Context) {
    egui::CentralPanel::default().show(ctx, |ui| {
        ui.vertical_centered(|ui| {
            ui.heading("🐍 RustySnake");
            ui.add_space(20.0);
            
            if ui.button("🎮 Start Game").clicked() {
                app.start_game();
            }
            
            if ui.button("🏆 Leaderboard").clicked() {
                app.current_screen = crate::app::state::AppScreen::Leaderboard;
            }
            
            if ui.button("⚙️ Settings").clicked() {
                app.current_screen = crate::app::state::AppScreen::Settings;
            }
            
            if ui.button("🚪 Quit").clicked() {
                ctx.send_viewport_cmd(egui::ViewportCommand::Close);
            }
            
            ui.add_space(30.0);
            ui.label("Controls: WASD or Arrow Keys");
            ui.label("Pause: ESC or P");
        });
    });
}
