use eframe::egui;

/// 数量のプラスとマイナスを操作する機能
pub fn qty_stepper(ui: &mut egui::Ui, qty: &mut u32, unit: &str) {
    ui.label("数量　　　");
    ui.add_space(32.0);
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

pub fn toggle_button(ui: &mut egui::Ui, on: &mut bool, text: &str) {
    let (fill, text_color) = if *on {
        (egui::Color32::from_rgb(225, 100, 0), egui::Color32::WHITE)
    } else {
        (egui::Color32::from_gray(225), egui::Color32::WHITE)
    };
    let label = if *on { format!("✓ {text}") } else { text.to_owned() };
    let button = egui::Button::new(egui::RichText::new(label).color(text_color))
        .fill(fill)
        .min_size(egui::vec2(160.0, 48.0));
    if ui.add(button).clicked() {
        *on = !*on;
    }
}