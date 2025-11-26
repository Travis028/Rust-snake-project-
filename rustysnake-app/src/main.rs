mod app;
mod game;
mod data;
mod utils;

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
        Box::new(|_cc| Box::<SnakeApp>::default()),
    )
}
