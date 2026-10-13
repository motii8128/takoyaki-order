use eframe::egui;

/// 数量のプラスとマイナスを操作する機能
pub fn qty_stepper(ui: &mut egui::Ui, qty: &mut u32, unit: &str) {
    ui.label("数量");
    if ui
        .add_enabled(*qty > 1, egui::Button::new("−").min_size(egui::vec2(56.0, 56.0)))
        .clicked()
    {
        *qty -= 1;
    }
    ui.label(egui::RichText::new(format!("{} {}", qty, unit)).size(30.0));
    if ui
        .add_enabled(*qty < 20, egui::Button::new("＋").min_size(egui::vec2(56.0, 56.0)))
        .clicked()
    {
        *qty += 1;
    }
}

