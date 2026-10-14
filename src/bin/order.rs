use eframe::egui;

use takoyaki_order::OrderApp;

fn main() -> eframe::Result {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([820.0, 560.0]),
        ..Default::default()
    };
    eframe::run_native(
        "Order",
        options,
        Box::new(|cc| Ok(Box::new(OrderApp::new(cc)))),
    )
}