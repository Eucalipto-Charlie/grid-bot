use crate::types::intent::TradeResult;

#[derive(Debug)]
pub enum Event {
    PriceCrossed {
        grid_index: usize,
    },

    TxConfirmed {
        result: TradeResult,
    },

    TxFailed {
        reason: String,
    },
}
