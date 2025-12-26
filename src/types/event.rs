use crate::types::intent::TradeResult;

#[derive(Debug)]
pub enum Event {
    PriceCrossed {
        grid_index: usize,
    },

    TxConfirmed {
        order_id: u64,
        result: TradeResult,
    },

    TxFailed {
        order_id: u64,
        reason: String,
    },
}
