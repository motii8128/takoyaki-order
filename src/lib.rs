// このアプリならではの型がそれぞれ定義されている
mod type_define;
use type_define::{Size, Sauce, CartItem};

// 注文データをTCP通信で送信するためのモジュール
mod tcp_connection;

// 見た目に関するモジュール
mod visual;

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
    // 入力中の商品
    size: Size,
    sauce: Sauce,
    aonori: bool,
    katsuobushi: bool,
    qty: u32,
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
            size: Size::S8,
            sauce: Sauce::Sauce,
            aonori: true,
            katsuobushi: true,
            qty: 1,
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

        // 右: カート
        egui::SidePanel::right("cart").min_width(420.0).show(ctx, |ui| {
            ui.heading("🛒 ご注文内容");
            ui.separator();

            let mut remove: Option<usize> = None;
            if self.cart.is_empty() {
                ui.label("まだ商品がありません");
            }
            for (i, item) in self.cart.iter().enumerate() {
                ui.horizontal(|ui| {
                    ui.vertical(|ui| {
                        ui.label(item.describe());
                        ui.label(format!(
                            "{} 円 × {} = {} 円",
                            item.size.price(),
                            item.qty,
                            item.subtotal()
                        ));
                    });
                    if ui.add_enabled(!sending, egui::Button::new("🗑")).clicked() {
                        remove = Some(i);
                    }
                });
                ui.separator();
            }
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
                        ui.spinner();
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

        // 中央: 注文フォーム
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("🐙 たこやき 注文受付");
            ui.separator();

            ui.label("個数");
            ui.horizontal(|ui| {
                for s in Size::ALL {
                    ui.radio_value(&mut self.size, s, format!("{}個 ({}円)", s.count(), s.price()));
                }
            });

            ui.add_space(8.0);
            ui.label("味");
            ui.horizontal(|ui| {
                for s in Sauce::ALL {
                    ui.radio_value(&mut self.sauce, s, s.label());
                }
            });

            ui.add_space(8.0);
            ui.label("トッピング");
            ui.horizontal(|ui| {
                ui.checkbox(&mut self.aonori, "青のり");
                ui.checkbox(&mut self.katsuobushi, "かつお節");
            });

            ui.add_space(8.0);
            ui.horizontal(|ui| {
                ui.label("数量");
                if ui
                    .add_enabled(self.qty > 1, egui::Button::new("−").min_size(egui::vec2(56.0, 56.0)))
                    .clicked()
                {
                    self.qty -= 1;
                }
                ui.label(egui::RichText::new(format!("{} 舟", self.qty)).size(30.0));
                if ui
                    .add_enabled(self.qty < 20, egui::Button::new("＋").min_size(egui::vec2(56.0, 56.0)))
                    .clicked()
                {
                    self.qty += 1;
                }
            });

            ui.add_space(12.0);
            if ui
                .add_enabled(
                    !sending,
                    egui::Button::new("＋ カートに追加").min_size(egui::vec2(240.0, 56.0)),
                )
                .clicked()
            {
                self.cart.push(CartItem {
                    size: self.size,
                    sauce: self.sauce,
                    aonori: self.aonori,
                    katsuobushi: self.katsuobushi,
                    qty: self.qty,
                });
                self.qty = 1;
                self.status = Status::None;
            }
        });
    }
}