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

use crate::type_define::Entry;
/// 注文1件のカード。ボタンが押されたら true を返す
pub fn entry_card(ui: &mut egui::Ui, e: &Entry, done: bool) -> bool {
    let mut clicked = false;
    egui::Frame::group(ui.style()).show(ui, |ui| {
        ui.set_width(ui.available_width());

        let time = e.received_at.split(' ').nth(1).unwrap_or("");
        let mut title = format!("#{}   {}", e.order_no, time);
        if let Some(t) = e.total {
            title += &format!("   {} 円", t);
        }
        if !e.saved {
            title += "  ⚠未保存";
        }
        let color = if done {
            ui.visuals().weak_text_color()
        } else {
            ui.visuals().strong_text_color()
        };
        ui.label(egui::RichText::new(title).strong().size(26.0).color(color));

        ui.label(format!(
            "たこやき {} 個 / ベビーカステラ {} 個",
            e.tako, e.castella
        ));
        for d in &e.details {
            ui.label(d);
        }

        let text = if done { "↩ 調理待ちに戻す" } else { "✅ 調理済みにする" };
        if ui
            .add(egui::Button::new(text).min_size(egui::vec2(260.0, 56.0)))
            .clicked()
        {
            clicked = true;
        }
    });
    clicked
}
