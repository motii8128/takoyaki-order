pub const SERVER_ADDR : [u8; 4] = [10, 42, 0, 1];
pub const SERVER_PORT : u16 = 64201;

use std::io::{Write, BufReader, BufRead};
use std::time::Duration;
use std::net::{TcpStream, SocketAddr, TcpListener};

use crate::type_define::OrderMsg;

pub fn send_order(msg: &OrderMsg) -> Result<(), String> {

    let server_ip = std::net::IpAddr::from(SERVER_ADDR);
    let socket_addr = SocketAddr::new(server_ip, SERVER_PORT);

    let mut stream = TcpStream::connect_timeout(&socket_addr, Duration::from_secs(3)).unwrap();

    stream.set_write_timeout(Some(Duration::from_secs(3))).map_err(|e| e.to_string())?;

    // 送信する文字列を作成
    let mut line = serde_json::to_string(msg).unwrap();
    line.push('\n');
    
    // TCP通信によりデータを送信
    stream.write_all(line.as_bytes()).unwrap();
    stream.flush().map_err(|e| e.to_string())?;
    Ok(())
}


//////////
// ここから受信側

use std::sync::mpsc::Sender;
use eframe::egui;

pub enum Event {
    Order(OrderMsg),
    Error(String),
}

pub fn start_listener(tx: Sender<Event>, ctx: egui::Context) {
    std::thread::spawn(move || {
        let server_ip = std::net::IpAddr::from(SERVER_ADDR);
        let socket_addr = SocketAddr::new(server_ip, SERVER_PORT);

        let listener = match TcpListener::bind(socket_addr) {
            Ok(l) => l,
            Err(e) => {
                let _ = tx.send(Event::Error(format!("{:?} で待ち受けできません: {e}", SERVER_ADDR)));
                ctx.request_repaint();
                return;
            }
        };
        for stream in listener.incoming() {
            if let Ok(s) = stream {
                let tx = tx.clone();
                let ctx = ctx.clone();
                std::thread::spawn(move || handle_conn(s, tx, ctx));
            }
        }
    });
}

/// 1接続ぶん：改行区切りのJSONを1行ずつ読む
pub fn handle_conn(stream: TcpStream, tx: Sender<Event>, ctx: egui::Context) {
    let _ = stream.set_read_timeout(Some(Duration::from_secs(10)));
    for line in BufReader::new(stream).lines() {
        let Ok(line) = line else { break };
        if line.trim().is_empty() {
            continue;
        }
        let ev = match serde_json::from_str::<OrderMsg>(&line) {
            Ok(m) if !m.items.is_empty() => Event::Order(m),
            Ok(_) => Event::Error("商品のない注文を受信しました".to_owned()),
            Err(e) => Event::Error(format!("受信データを解釈できません: {e}")),
        };
        let _ = tx.send(ev);
        ctx.request_repaint();
    }
}