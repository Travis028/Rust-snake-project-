use eframe::egui;
use crate::app::state::SnakeApp;

pub fn show(app: &mut SnakeApp, ctx: &egui::Context) {
    // Handle keyboard input
    if ctx.input(|i| i.key_pressed(egui::Key::Escape) || i.key_pressed(egui::Key::P)) {
        app.pause_game();
    }
    
    if ctx.input(|i| i.key_pressed(egui::Key::W) || i.key_pressed(egui::Key::ArrowUp)) {
        app.game.change_direction((0, -1));
    }
    if ctx.input(|i| i.key_pressed(egui::Key::S) || i.key_pressed(egui::Key::ArrowDown)) {
        app.game.change_direction((0, 1));
    }
    if ctx.input(|i| i.key_pressed(egui::Key::A) || i.key_pressed(egui::Key::ArrowLeft)) {
        app.game.change_direction((-1, 0));
    }
    if ctx.input(|i| i.key_pressed(egui::Key::D) || i.key_pressed(egui::Key::ArrowRight)) {
        app.game.change_direction((1, 0));
    }
    
    // Update game state
    app.game.update();
    
    if app.game.game_over {
        app.game_over();
    }
    
    // Display game
    egui::CentralPanel::default().show(ctx, |ui| {
        ui.horizontal(|ui| {
            ui.label(format!("Score: {}", app.game.score));
            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                if ui.button("⏸️ Pause").clicked() {
                    app.pause_game();
                }
            });
        });
        
        ui.add_space(10.0);
        
        // Draw game board
        draw_game_board(ui, &app.game);
    });
}

fn draw_game_board(ui: &mut egui::Ui, game: &crate::game::core::Game) {
    let cell_size = 20.0;
    let (response, painter) = ui.allocate_painter(
        egui::Vec2::new(
            game.width as f32 * cell_size,
            game.height as f32 * cell_size,
        ),
        egui::Sense::hover(),
    );
    
    // Draw background
    painter.rect_filled(response.rect, 0.0, egui::Color32::from_rgb(30, 30, 46));
    
    // Draw snake
    for (i, pos) in game.snake.iter().enumerate() {
        let color = if i == 0 {
            egui::Color32::from_rgb(0, 255, 0) // Head - green
        } else {
            egui::Color32::from_rgb(0, 200, 0) // Body - darker green
        };
        
        let rect = egui::Rect::from_min_size(
            response.rect.min + egui::Vec2::new(pos.x as f32 * cell_size, pos.y as f32 * cell_size),
            egui::Vec2::new(cell_size - 1.0, cell_size - 1.0),
        );
        painter.rect_filled(rect, 3.0, color);
    }
    
    // Draw food
    let food_rect = egui::Rect::from_min_size(
        response.rect.min + egui::Vec2::new(game.food.x as f32 * cell_size, game.food.y as f32 * cell_size),
        egui::Vec2::new(cell_size - 1.0, cell_size - 1.0),
    );
    painter.rect_filled(food_rect, 3.0, egui::Color32::from_rgb(255, 0, 0));
    
    // Game over overlay
    if game.game_over {
        let overlay_rect = response.rect;
        painter.rect_filled(overlay_rect, 0.0, egui::Color32::from_rgba_unmultiplied(0, 0, 0, 180));
        
        let text = "Game Over! Press P for menu";
        let galley = ui.painter().layout(
            text.to_string(),
            egui::FontId::proportional(20.0),
            egui::Color32::WHITE,
            f32::INFINITY,
        );
        let text_pos = overlay_rect.center() - galley.size() / 2.0;
        painter.galley(text_pos, galley, egui::Color32::WHITE);
    }
}
