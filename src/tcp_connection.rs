pub const SERVER_ADDR : [u8; 4] = [192, 168, 11, 4];
pub const SERVER_PORT : u16 = 64201;

use std::io::Write;
use std::time::Duration;
use std::net::{TcpStream, SocketAddr};

use serde::Serialize;

use crate::type_define::CartItem;


// 別のPCへ送信するデータ形式を定義する
#[derive(Serialize)]
struct ItemMsg {
    name: String,
    pieces: u32,
    #[serde(skip_serializing_if = "Option::is_none")]
    sauce: Option<String>,
    toppings: Vec<String>,
    qty: u32,
    subtotal: u32,
}

fn to_msg(cart_item : &CartItem) -> ItemMsg {
    match cart_item {
        CartItem::Takoyaki { size, sauce, aonori, katsuobushi, qty } => {
            let mut toppings = vec![];
            if *aonori {
                toppings.push("青のり".to_owned());
            }
            if *katsuobushi {
                toppings.push("かつお節".to_owned());
            }
            ItemMsg {
                name: "たこやき".to_owned(),
                pieces: size.count(),
                sauce: Some(sauce.label().to_owned()),
                toppings,
                qty: *qty,
                subtotal: cart_item.subtotal(),
            }
        }
        CartItem::Castella { size, choco, honey, qty } => {
            let mut toppings = vec![];
            if *choco {
                toppings.push("チョコソース".to_owned());
            }
            if *honey {
                toppings.push("はちみつ".to_owned());
            }
            ItemMsg {
                name: "ベビーカステラ".to_owned(),
                pieces: size.count(),
                sauce: None,
                toppings,
                qty: *qty,
                subtotal: cart_item.subtotal(),
            }
        }
    }
}

#[derive(Serialize)]
pub struct OrderMsg {
    order_no: u32,
    items: Vec<ItemMsg>,
    total: u32,
}

impl OrderMsg {
    pub fn new(order_no : u32, cart : &[CartItem]) -> Self
    {
        OrderMsg {
            order_no,
            total: cart.iter().map(|i| i.subtotal()).sum(),
            items: cart.iter().map(|i| to_msg(i)).collect(),
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