use eframe::egui;

pub fn setup_style(ctx: &egui::Context) {
    use egui::{FontId, TextStyle};
    ctx.style_mut(|style| {
        let v = &mut style.visuals;
        *v = egui::Visuals::dark(); // ライトテーマ（標準はダーク）

        v.panel_fill = egui::Color32::from_rgb(255, 255, 230); // パネルの背景
        v.window_fill = egui::Color32::from_rgb(0, 248, 0);

        // チェック系の見た目設定
        v.selection.bg_fill = egui::Color32::from_rgb(255, 255,255); // ラジオ/チェックの選択色
        v.selection.stroke = egui::Stroke::new(2.0_f32, egui::Color32::WHITE); // ✓ マークの色

        // ボタンの見た目設定
        v.widgets.inactive.weak_bg_fill = egui::Color32::from_rgb(100, 225, 0); // ボタンの通常色
        v.widgets.hovered.weak_bg_fill = egui::Color32::from_rgb(50, 120, 0); // 乗せたとき
        v.widgets.active.weak_bg_fill = egui::Color32::from_rgb(50, 120, 0); // 押している間

        // テキストの色
        v.override_text_color = Some(egui::Color32::from_rgb(0, 0, 0)); // 文字色

        style.text_styles = [
            (TextStyle::Heading, FontId::proportional(64.0)),
            (TextStyle::Body, FontId::proportional(22.0)),
            (TextStyle::Button, FontId::proportional(40.0)),
            (TextStyle::Small, FontId::proportional(16.0)),
            (TextStyle::Monospace, FontId::monospace(20.0)),
        ]
        .into();

        style.spacing.button_padding = egui::vec2(16.0, 10.0);
        style.spacing.item_spacing = egui::vec2(24.0, 38.0);
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
        "/home/motii/Downloads/minn_tyou_tai/ipam.ttc"
        // "/usr/share/fonts/opentype/noto/NotoSansCJK-Regular.ttc",
        // "/usr/share/fonts/truetype/noto/NotoSansCJK-Regular.ttc",
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