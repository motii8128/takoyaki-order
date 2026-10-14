use serde::{Deserialize, Serialize};

const TAKOYAKI_VALUE : u32 = 400;
const KASUTERA_VALUE : u32 = 200;

/// たこやきの注文数を選ぶ
#[derive(Clone, Copy, PartialEq)]
pub enum Size {
    S4,
}
impl Size {
    pub const ALL: [Size; 1] = [Size::S4];
    pub fn count(self) -> u32 {
        match self {
            Size::S4 => 4,
        }
    }
    pub fn price(self) -> u32 {
        match self {
            Size::S4 => TAKOYAKI_VALUE,
        }
    }
}


// ソースかソース＆マヨか
#[derive(Clone, Copy, PartialEq)]
pub enum Sauce {
    Nasi,
    Sauce,
    Mayo,
}

impl Sauce {
    pub const ALL: [Sauce; 3] = [Sauce::Nasi, Sauce::Sauce, Sauce::Mayo];
    pub fn label(self) -> &'static str {
        match self {
            Sauce::Nasi => "なし",
            Sauce::Sauce => "ソース",
            Sauce::Mayo => "ソース＋マヨ",
        }
    }
}



// 各注文が入るカート
#[derive(Clone)]
pub enum CartItem {
    Takoyaki {
        size: Size,
        sauce: Sauce,
        aonori: bool,
        katsuobushi: bool,
        qty: u32,
    },
    Castella {
        size: Size,
        choco: bool,
        honey: bool,
        qty: u32,
    },
}

impl CartItem {
    pub fn qty(&self) -> u32 {
        match self {
            CartItem::Takoyaki { qty, .. } | CartItem::Castella { qty, .. } => *qty,
        }
    }

    pub fn unit_price(&self) -> u32 {
        match self {
            CartItem::Takoyaki { .. } => TAKOYAKI_VALUE,
            CartItem::Castella { .. } => KASUTERA_VALUE,
        }
    }

    pub fn subtotal(&self) -> u32 {
        self.unit_price() * self.qty()
    }

    pub fn describe(&self) -> String {
        match self {
            CartItem::Takoyaki { size, sauce, aonori, katsuobushi, .. } => {
                let mut s = format!("たこやき {}個 / {}", size.count(), sauce.label());
                if *aonori {
                    s += " / 青のり";
                }
                if *katsuobushi {
                    s += " / かつお節";
                }
                s
            }
            CartItem::Castella { size, choco, honey, .. } => {
                let mut s = format!("ベビーカステラ {}個", size.count());
                if *choco {
                    s += " / 砂糖";
                }
                if *honey {
                    s += " / はちみつ";
                }
                s
            }
        }
    }
}

// 別のPCへ送信するデータ形式を定義する
#[derive(Serialize, Deserialize, Clone)]
pub struct ItemMsg {
    pub name: String,
    pub pieces: u32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sauce: Option<String>,
    pub toppings: Vec<String>,
    pub qty: u32,
    pub subtotal: u32,
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
                toppings.push("砂糖".to_owned());
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

#[derive(Serialize, Deserialize, Clone)]
pub struct OrderMsg {
    pub order_no: u32,
    pub items: Vec<ItemMsg>,
    pub total: u32,
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


#[derive(Serialize, Deserialize)]
pub struct Row {
    #[serde(rename = "時間")]
    pub received_at: String,
    pub order_no: u32,
    #[serde(rename = "たこやき個数")]
    pub takoyaki: u32,
    #[serde(rename = "ベビーカステラ個数")]
    pub castella: u32,
}

/// 画面に出す1件の注文
pub struct Entry {
    pub received_at: String,
    pub order_no: u32,
    pub tako: u32,          // たこやきの個数（個数 × 数量）
    pub castella: u32,      // ベビーカステラの個数（個数 × 数量）
    pub total: Option<u32>, // 金額（再起動後に復元した注文では None）
    pub details: Vec<String>, // 味・トッピングなどの明細（再起動後は空）
    pub done: bool,         // 調理済み
    pub saved: bool,        // CSVに保存済み
}

impl Entry {
    pub fn to_row(&self) -> Row {
        Row {
            received_at: self.received_at.clone(),
            order_no: self.order_no,
            takoyaki: self.tako,
            castella: self.castella,
        }
    }
}

pub fn detail_text(it: &ItemMsg) -> String {
    let mut s = format!("{} {}個", it.name, it.pieces);
    if let Some(sauce) = &it.sauce {
        s += &format!(" / {sauce}");
    }
    if !it.toppings.is_empty() {
        s += &format!(" / {}", it.toppings.join("・"));
    }
    s + &format!("  × {}  ", it.qty)
}

pub fn to_entry(msg: OrderMsg) -> Entry {
    let mut tako = 0;
    let mut castella = 0;
    let mut details = vec![];
    for it in &msg.items {
        // let pieces = it.pieces * it.qty;
        match it.name.as_str() {
            "たこやき" => tako += it.qty,
            "ベビーカステラ" => castella += it.qty,
            _ => {}
        }
        details.push(detail_text(it));
    }
    Entry {
        received_at: chrono::Local::now().format("%Y-%m-%d %H:%M:%S").to_string(),
        order_no: msg.order_no,
        tako,
        castella,
        total: Some(msg.total),
        details,
        done: false,
        saved: false,
    }
}