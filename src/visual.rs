use eframe::egui;

pub fn setup_style(ctx: &egui::Context) {
    use egui::{FontId, TextStyle};
    ctx.style_mut(|style| {
        style.text_styles = [
            (TextStyle::Heading, FontId::proportional(32.0)),
            (TextStyle::Body, FontId::proportional(22.0)),
            (TextStyle::Button, FontId::proportional(22.0)),
            (TextStyle::Small, FontId::proportional(16.0)),
            (TextStyle::Monospace, FontId::monospace(20.0)),
        ]
        .into();

        style.spacing.button_padding = egui::vec2(16.0, 10.0);
        style.spacing.item_spacing = egui::vec2(12.0, 12.0);
        style.spacing.interact_size.y = 40.0;
        style.spacing.icon_width = 28.0;
        style.spacing.icon_width_inner = 16.0;
    });
}

/// egui標準フォントは日本語を含まないので、OSのフォントを読み込む
pub fn setup_fonts(ctx: &egui::Context) {
    let candidates = [
        "C:\\Windows\\Fonts\\meiryo.ttc",
        "C:\\Windows\\Fonts\\YuGothR.ttc",
        "/System/Library/Fonts/ヒラギノ角ゴシック W3.ttc",
        "/usr/share/fonts/opentype/noto/NotoSansCJK-Regular.ttc",
        "/usr/share/fonts/truetype/noto/NotoSansCJK-Regular.ttc",
    ];
    for path in candidates {
        if let Ok(bytes) = std::fs::read(path) {
            let mut fonts = egui::FontDefinitions::default();
            fonts
                .font_data
                .insert("jp".to_owned(), egui::FontData::from_owned(bytes));
            fonts
                .families
                .get_mut(&egui::FontFamily::Proportional)
                .unwrap()
                .insert(0, "jp".to_owned());
            fonts
                .families
                .get_mut(&egui::FontFamily::Monospace)
                .unwrap()
                .push("jp".to_owned());
            ctx.set_fonts(fonts);
            return;
        }
    }
    eprintln!("日本語フォントが見つかりませんでした（文字化けします）");
}