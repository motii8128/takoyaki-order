const TAKOYAKI_VALUE : u32 = 250;
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
                    s += " / チョコソース";
                }
                if *honey {
                    s += " / はちみつ";
                }
                s
            }
        }
    }
}