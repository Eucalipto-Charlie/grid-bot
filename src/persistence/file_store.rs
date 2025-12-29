use std::{
    fs::{OpenOptions, File},
    io::{BufRead, BufReader, Write},
    path::PathBuf,
};

use serde_json;

use crate::types::intent::{TradeIntent, TradeResult};
use crate::persistence::trade_store::TradeStore;

pub struct FileTradeStore {
    intent_path: PathBuf,
    result_path: PathBuf,
}

impl FileTradeStore {
    pub fn new(base_dir: impl Into<PathBuf>) -> Self {
        let base = base_dir.into();
        std::fs::create_dir_all(&base).expect("create data dir failed");

        Self {
            intent_path: base.join("trade_intents.log"),
            result_path: base.join("trade_results.log"),
        }
    }

    fn append_json<T: serde::Serialize>(
        path: &PathBuf,
        value: &T,
    ) {
        let mut file = OpenOptions::new()
            .create(true)
            .append(true)
            .open(path)
            .expect("open file failed");

        let line = serde_json::to_string(value)
            .expect("serialize failed");

        writeln!(file, "{line}")
            .expect("write failed");
    }

    fn read_json_lines<T: for<'de> serde::Deserialize<'de>>(
        path: &PathBuf,
    ) -> Vec<T> {
        if !path.exists() {
            return vec![];
        }

        let file = File::open(path).expect("open file failed");
        let reader = BufReader::new(file);

        reader
            .lines()
            .filter_map(|line| {
                let line = line.ok()?;
                serde_json::from_str(&line).ok()
            })
            .collect()
    }
}

impl TradeStore for FileTradeStore {
    fn record_intent(&mut self, intent: &TradeIntent) {
        Self::append_json(&self.intent_path, intent);
    }

    fn record_result(&mut self, result: &TradeResult) {
        Self::append_json(&self.result_path, result);
    }

    fn list_intents(&self) -> Vec<TradeIntent> {
        Self::read_json_lines(&self.intent_path)
    }

    fn list_results(&self) -> Vec<TradeResult> {
        Self::read_json_lines(&self.result_path)
    }
}
