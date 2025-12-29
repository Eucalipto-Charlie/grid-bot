use crate::persistence::trade_store::TradeStore;
use crate::types::intent::{TradeIntent, TradeResult, Side};

pub struct MemoryTradeStore {
    intents: Vec<TradeIntent>,
    results: Vec<TradeResult>,
}

#[derive(Debug)]
struct TradeRow {
    order_id: u64,
    grid_index: usize,
    side: Side,
    price: f64,
    amount: f64,
    status: String,
}


impl MemoryTradeStore {
    pub fn new() -> Self {
        Self {
            intents: Vec::new(),
            results: Vec::new(),
        }
    }
}

impl TradeStore for MemoryTradeStore {
    fn record_intent(&mut self, intent: &TradeIntent) {
        self.intents.push(intent.clone());
    }

    fn record_result(&mut self, result: &TradeResult) {
        self.results.push(result.clone());
    }

    fn list_intents(&self) -> Vec<TradeIntent> {
        self.intents.clone()
    }

    fn list_results(&self) -> Vec<TradeResult> {
        self.results.clone()
    }
}

impl MemoryTradeStore {
    pub fn dump(&self) {
        println!("\n====== Trade Store Dump ======");

        println!("\n--- TradeIntents ({} items) ---", self.intents.len());
        for intent in &self.intents {
            println!("{:?}", intent);
        }

        println!("\n--- TradeResults ({} items) ---", self.results.len());
        for result in &self.results {
            println!("{:?}", result);
        }

        println!("====== End Dump ======\n");
    }

    pub fn dump_table(&self) {
        println!("\n================= Trade Ledger =================");
        println!(
            "{:<20} {:<6} {:<6} {:<10} {:<8} {:<10}",
            "OrderId", "Grid", "Side", "Price", "Amount", "Status"
        );
        println!("{}", "-".repeat(70));

        // 先放入所有 Intent（初始为 Submitted）
        let mut rows: Vec<TradeRow> = self
            .intents
            .iter()
            .map(|i| TradeRow {
                order_id: i.order_id,
                grid_index: i.grid_index,
                side: i.side,
                price: i.price,
                amount: i.amount,
                status: "SUBMITTED".to_string(),
            })
            .collect();

        // 用 Result 覆盖状态
        for r in &self.results {
            if let Some(row) = rows.iter_mut().find(|x| x.order_id == r.order_id) {
                if r.success {
                    row.status = "FILLED".to_string();
                } else {
                    row.status = format!("FAILED({})", r.reason.clone().unwrap_or_default());
                }
            } else {
                // 如果没有对应的 Intent，直接添加 Result 记录
                rows.push(TradeRow {
                    order_id: r.order_id,
                    grid_index: r.grid_index,
                    side: r.side,
                    price: r.price,
                    amount: r.amount,
                    status: if r.success {
                        "FILLED".to_string()
                    } else {
                        "FAILED".to_string()
                    },
                });
            }
        }

        // 打印
        for row in rows {
            println!(
                "{:<20} {:<6} {:<6} {:<10.2} {:<8.2} {:<10}",
                row.order_id,
                row.grid_index,
                format!("{:?}", row.side),
                row.price,
                row.amount,
                row.status
            );
        }

        println!("================================================\n");
    }
}