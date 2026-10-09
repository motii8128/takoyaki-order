pub const SERVER_ADDR : [u8; 4] = [192, 168, 11, 4];
pub const SERVER_PORT : u16 = 64201;

use std::io::Write;
use std::time::Duration;
use std::net::{TcpStream, SocketAddr};

use serde::Serialize;

use crate::type_define::CartItem;


// 別のPCへ送信するデータ形式を定義する
#[derive(Serialize)]
pub struct OrderItemMsg {
    name: String,
    pieces: u32,
    sauce: String,
    aonori: bool,
    katsuobushi: bool,
    qty: u32,
    subtotal: u32,
}

#[derive(Serialize)]
pub struct OrderMsg {
    order_no: u32,
    items: Vec<OrderItemMsg>,
    total: u32,
}

impl OrderMsg {
    pub fn new(order_no : u32, cart : &[CartItem]) -> Self
    {
        OrderMsg {
            order_no,
            total: cart.iter().map(|i| i.subtotal()).sum(),
            items: cart
                .iter()
                .map(|i| OrderItemMsg {
                    name: "たこやき".to_string(),
                    pieces: i.size.count(),
                    sauce: i.sauce.label().to_owned(),
                    aonori: i.aonori,
                    katsuobushi: i.katsuobushi,
                    qty: i.qty,
                    subtotal: i.subtotal(),
                })
                .collect(),
        }
    }
}

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