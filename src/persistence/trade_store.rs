use crate::types::intent::{TradeIntent, TradeResult};

pub trait TradeStore: Send {
    fn record_intent(&mut self, intent: &TradeIntent);
    fn record_result(&mut self, result: &TradeResult);

    fn list_intents(&self) -> Vec<TradeIntent>;
    fn list_results(&self) -> Vec<TradeResult>;
}
