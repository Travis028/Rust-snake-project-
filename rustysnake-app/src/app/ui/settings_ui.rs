use eframe::egui;
use crate::app::state::SnakeApp;

pub fn show(app: &mut SnakeApp, ctx: &egui::Context) {
    egui::CentralPanel::default().show(ctx, |ui| {
        ui.heading("⚙️ Settings");
        
        ui.add_space(20.0);
        
        // Volume control
        ui.label("Volume:");
        ui.add(egui::Slider::new(&mut app.config.volume, 0.0..=1.0)
            .show_value(false)
            .text("🔊"));
        
        // Update volume when slider changes
        if ui.button("Apply Volume").clicked() {
            app.update_volume();
        }
        
        ui.add_space(20.0);
        
        // Rest of the settings...
    });
}
