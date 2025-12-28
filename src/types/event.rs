use crate::types::intent::TradeResult;

#[derive(Debug)]
pub enum Event {
    PriceTick {
        price: f64,
    },

    PriceCrossed {
        grid_index: usize,
    },

    TxSubmitted {
        order_id: u64,
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
