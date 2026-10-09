use eframe::egui;

use takoyaki_order::App;

fn main() -> eframe::Result {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([820.0, 560.0]),
        ..Default::default()
    };
    eframe::run_native(
        "たこやき注文",
        options,
        Box::new(|cc| Ok(Box::new(App::new(cc)))),
    )
}