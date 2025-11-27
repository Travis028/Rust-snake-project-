use eframe::egui;

pub struct ThemeManager {
    pub current_theme: String,
}

impl Default for ThemeManager {
    fn default() -> Self {
        Self {
            current_theme: "Dark".to_string(),
        }
    }
}

impl ThemeManager {
    pub fn apply_theme(&self, ctx: &egui::Context) {
        match self.current_theme.as_str() {
            "Dark" => self.apply_dark_theme(ctx),
            "Light" => self.apply_light_theme(ctx),
            _ => self.apply_dark_theme(ctx),
        }
    }
    
    fn apply_dark_theme(&self, ctx: &egui::Context) {
        let mut style = (*ctx.style()).clone();
        style.visuals = egui::Visuals::dark();
        ctx.set_style(style);
    }
    
    fn apply_light_theme(&self, ctx: &egui::Context) {
        let mut style = (*ctx.style()).clone();
        style.visuals = egui::Visuals::light();
        ctx.set_style(style);
    }
}