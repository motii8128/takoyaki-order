use crate::type_define::{Entry, Row};

use std::fs::OpenOptions;
use std::io::Write;

pub const CSV_PATH: &str = "orders.csv";

/// 起動時に既存のCSVを読み込む（過去の注文なので調理済みとして扱う）
pub fn load_csv() -> Vec<Entry> {
    let Ok(mut rdr) = csv::Reader::from_path(CSV_PATH) else {
        return vec![];
    };
    rdr.deserialize::<Row>()
        .filter_map(|r| r.ok())
        .map(|r| Entry {
            received_at: r.received_at,
            order_no: r.order_no,
            tako: r.takoyaki,
            castella: r.castella,
            total: None,
            details: vec![],
            done: true,
            saved: true,
        })
        .collect()
}

/// CSVに追記する（新規ファイルならBOMとヘッダーを書く）
pub fn append_csv(rows: &[Row]) -> Result<(), String> {
    let is_new = std::fs::metadata(CSV_PATH).map(|m| m.len() == 0).unwrap_or(true);
    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(CSV_PATH)
        .map_err(|e| format!("CSVを開けません: {e}"))?;
    if is_new {
        // Excelで文字化けしないよう UTF-8 BOM を付ける
        file.write_all(&[0xEF, 0xBB, 0xBF]).map_err(|e| e.to_string())?;
    }
    let mut wtr = csv::WriterBuilder::new().has_headers(false).from_writer(file);
    if is_new {
        wtr.write_record(["時間", "order_no", "たこやき個数", "ベビーカステラ個数"])
            .map_err(|e| e.to_string())?;
    }
    for r in rows {
        wtr.serialize(r).map_err(|e| e.to_string())?;
    }
    wtr.flush().map_err(|e| format!("CSVへの書き込みに失敗: {e}"))
}