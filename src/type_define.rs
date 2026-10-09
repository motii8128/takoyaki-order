const TAKOYAKI_4_VALUE : u32 = 450;
const TAKOYAKI_8_VALUE : u32 = 800;

/// たこやきの注文数を選ぶ
#[derive(Clone, Copy, PartialEq)]
pub enum Size {
    S4,
    S8,
}
impl Size {
    pub const ALL: [Size; 2] = [Size::S4, Size::S8];
    pub fn count(self) -> u32 {
        match self {
            Size::S4 => 4,
            Size::S8 => 8,
        }
    }
    pub fn price(self) -> u32 {
        match self {
            Size::S4 => TAKOYAKI_4_VALUE,
            Size::S8 => TAKOYAKI_8_VALUE,
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
pub struct CartItem {
    pub size: Size,
    pub sauce: Sauce,
    pub aonori: bool,
    pub katsuobushi: bool,
    pub qty: u32,
}

impl CartItem {
    pub fn subtotal(&self) -> u32 {
        self.size.price() * self.qty
    }
    pub fn describe(&self) -> String {
        let mut s = format!("たこやき {}個 / {}", self.size.count(), self.sauce.label());
        if self.aonori {
            s += " / 青のり";
        }
        if self.katsuobushi {
            s += " / かつお節";
        }
        s
    }
}