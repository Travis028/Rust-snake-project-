mod app;
mod game;
mod data;
mod utils;
mod audio;

use app::SnakeApp;
use eframe::egui;

fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([800.0, 600.0])
            .with_title("RustySnake - Full Application"),
        ..Default::default()
    };
    
    eframe::run_native(
        "RustySnake",
        options,
        Box::new(|cc| {
            // Initialize audio manager
            let audio_manager = std::sync::Arc::new(std::sync::Mutex::new(audio::AudioManager::new()));
            let app = SnakeApp::new(audio_manager);
            Box::new(app)
        }),
    )
}
