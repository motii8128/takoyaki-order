// このアプリならではの型がそれぞれ定義されている
mod type_define;
use type_define::{Size, Sauce, CartItem};

// 注文データをTCP通信で送信するためのモジュール
mod tcp_connection;

// 見た目に関するモジュール
mod visual;

// UIの機能を抽象化して実装したモジュール
mod ui_widgets;

use eframe::egui;

use std::sync::mpsc::{channel, Receiver, Sender};

use crate::tcp_connection::OrderMsg;


enum Status
{
    None,
    Sending,        // 送信中
    Done(u32),      // 送信成功した注文番号
    Failed(String)  // 送信失敗
}

pub struct App {
    t_size: Size,
    t_sauce: Sauce,
    t_aonori: bool,
    t_katsuobushi: bool,
    t_qty: u32,
    // ベビーカステラの入力中の内容
    c_size: Size,
    c_choco: bool,
    c_honey: bool,
    c_qty: u32,
    // カート
    cart: Vec<CartItem>,
    next_no: u32,
    status: Status,
    // 送信スレッドからの結果受け取り
    tx: Sender<Result<(), String>>,
    rx: Receiver<Result<(), String>>,
}

impl App {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        visual::setup_fonts(&cc.egui_ctx);
        visual::setup_style(&cc.egui_ctx);
        let (tx, rx) = channel();
        Self {
            t_size: Size::S4,
            t_sauce: Sauce::Sauce,
            t_aonori: true,
            t_katsuobushi: true,
            t_qty: 1,
            c_size: Size::S4,
            c_choco: false,
            c_honey: false,
            c_qty: 1,
            cart: vec![],
            next_no: 1,
            status: Status::None,
            tx,
            rx,
        }
    }

    fn cart_total(&self) -> u32 {
        self.cart.iter().map(|i| i.subtotal()).sum()
    }

    fn start_send(&mut self, ctx: &egui::Context) {
        let msg = OrderMsg::new(self.next_no, &self.cart);
        self.status = Status::Sending;
        let tx = self.tx.clone();
        let ctx = ctx.clone();
        std::thread::spawn(move || {
            let _ = tx.send(tcp_connection::send_order(&msg));
            ctx.request_repaint();
        });
    }
}

impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {

        if ctx.input(|i| i.key_pressed(egui::Key::F11)) {
            let is_full = ctx.input(|i| i.viewport().fullscreen.unwrap_or(false));
            ctx.send_viewport_cmd(egui::ViewportCommand::Fullscreen(!is_full));
        }
        // Esc: 全画面を解除
        if ctx.input(|i| i.key_pressed(egui::Key::Escape)) {
            ctx.send_viewport_cmd(egui::ViewportCommand::Fullscreen(false));
        }

        // 送信結果の受取
        while let Ok(result) = self.rx.try_recv() {
            match result {
                Ok(()) => {
                    // 成功したときだけカートを空にして番号を進める
                    self.status = Status::Done(self.next_no);
                    self.cart.clear();
                    self.next_no += 1;
                }
                Err(e) => self.status = Status::Failed(e), // カートは残す
            }
        }

        let sending = matches!(self.status, Status::Sending);

        ////////////////////////////////////////////////////////////////////////////////// 右: カート
        egui::SidePanel::right("cart").min_width(420.0).show(ctx, |ui| {
            ui.heading("ご注文内容");
            ui.separator();

            let mut remove: Option<usize> = None;
            if self.cart.is_empty() {
                ui.label("まだ商品がありません");
            }
            egui::ScrollArea::vertical()
                .max_height(ui.available_height() - 200.0)
                .show(ui, |ui| {
                    for (i, item) in self.cart.iter().enumerate() {
                        ui.horizontal(|ui| {
                            ui.vertical(|ui| {
                                ui.label(item.describe());
                                ui.label(format!(
                                    "{} 円 × {} = {} 円",
                                    item.unit_price(),
                                    item.qty(),
                                    item.subtotal()
                                ));
                            });
                            let trash = egui::Button::new(
                                egui::RichText::new("🗑").color(egui::Color32::WHITE),
                            )
                            .fill(egui::Color32::from_rgb(200, 40, 40));
                            if ui.add_enabled(!sending, trash).clicked() {
                                remove = Some(i);
                            }
                        });
                        ui.separator();
                    }
                });
            if let Some(i) = remove {
                self.cart.remove(i);
            }

            ui.heading(format!("合計: {} 円", self.cart_total()));
            ui.add_space(8.0);

            let can_send = !self.cart.is_empty() && !sending;
            if ui
                .add_enabled(
                    can_send,
                    egui::Button::new("✅ 注文を確定する").min_size(egui::vec2(280.0, 64.0)),
                )
                .clicked()
            {
                self.start_send(ctx);
            }

            ui.add_space(8.0);
            match &self.status {
                Status::None => {}
                Status::Sending => {
                    ui.horizontal(|ui| {
                        let sp = egui::widgets::Spinner::new().color(egui::Color32::BLACK);
                        ui.add(sp);
                        ui.label("送信中...");
                    });
                }
                Status::Done(no) => {
                    ui.colored_label(
                        egui::Color32::from_rgb(0, 150, 0),
                        format!("注文番号 {} を送信しました！", no),
                    );
                }
                Status::Failed(e) => {
                    ui.colored_label(egui::Color32::RED, format!("送信できませんでした: {e}"));
                    ui.label("もう一度「注文を確定する」を押してください");
                }
            }
        });

        ////////////////////////////////////////////////////////////////////////////////// 中央: 注文フォーム
        egui::CentralPanel::default().show(ctx, |ui| {
            egui::ScrollArea::vertical().show(ui, |ui| {
                // ===== たこやき（左上）=====
                ui.heading("🐙 たこやき");
                ui.separator();

                ui.horizontal(|ui| {
                    ui.label("個数　　　");
                    ui.add_space(32.0);
                    for s in Size::ALL {
                        ui.radio_value(&mut self.t_size, s, format!("{}個 ({}円)", s.count(), s.price()));
                    }
                });
                ui.horizontal(|ui| {
                    ui.label("味　　　　");
                    ui.add_space(32.0);
                    for s in Sauce::ALL {
                        let text = if s == self.t_sauce
                        {
                            egui::RichText::new(s.label()).color(egui::Color32::from_rgb(100, 225, 0))
                        }
                        else {
                            egui::RichText::new(s.label()).color(egui::Color32::BLACK)
                        };
                        ui.radio_value(&mut self.t_sauce, s, text);
                    }
                });
                ui.horizontal(|ui| {
                    ui.label("トッピング");
                    ui.add_space(32.0);
                    ui_widgets::toggle_button(ui, &mut self.t_aonori, "青のり");
                    ui_widgets::toggle_button(ui, &mut self.t_katsuobushi, "かつお節");
                });
                ui.horizontal(|ui| {
                    ui_widgets::qty_stepper(ui, &mut self.t_qty, "個");
                    ui.add_space(20.0);
                    if ui
                        .add_enabled(
                            !sending,
                            egui::Button::new("＋ カートに追加").min_size(egui::vec2(240.0, 56.0)),
                        )
                        .clicked()
                    {
                        self.cart.push(CartItem::Takoyaki {
                            size: self.t_size,
                            sauce: self.t_sauce,
                            aonori: self.t_aonori,
                            katsuobushi: self.t_katsuobushi,
                            qty: self.t_qty,
                        });
                        self.t_qty = 1;
                        self.status = Status::None;
                    }
                });

                ui.add_space(16.0);
                ui.separator();
                ui.add_space(16.0);

                // ===== ベビーカステラ（左下）=====
                ui.heading("🍰 ベビーカステラ");
                ui.separator();

                ui.horizontal(|ui| {
                    ui.label("個数　　　");
                    ui.add_space(32.0);
                    for s in Size::ALL {
                        ui.radio_value(&mut self.c_size, s, format!("{}個 ({}円)", s.count(), s.price()));
                    }
                });
                // ui.horizontal(|ui| {
                //     ui.label("トッピング");
                //     ui.checkbox(&mut self.c_choco, "チョコソース");
                //     ui.checkbox(&mut self.c_honey, "はちみつ");
                // });
                ui.horizontal(|ui| {
                    ui_widgets::qty_stepper(ui, &mut self.c_qty, "個");
                    ui.add_space(20.0);
                    if ui
                        .add_enabled(
                            !sending,
                            egui::Button::new("＋ カートに追加").min_size(egui::vec2(240.0, 56.0)),
                        )
                        .clicked()
                    {
                        self.cart.push(CartItem::Castella {
                            size: self.c_size,
                            choco: self.c_choco,
                            honey: self.c_honey,
                            qty: self.c_qty,
                        });
                        self.c_qty = 1;
                        self.status = Status::None;
                    }
                });
            });
        });
    }
}