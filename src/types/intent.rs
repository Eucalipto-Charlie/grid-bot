#[derive(Debug, Clone, Copy)]
pub enum Side {
    Buy,
    Sell,
}

#[derive(Debug, Clone)]
pub struct TradeIntent {
    pub order_id: u64,
    pub grid_index: usize,

    pub side: Side,
    pub price: f64,
    pub amount: f64,
}


#[derive(Debug)]
pub struct TradeResult {
    pub order_id: u64,
    pub grid_index: usize,

    pub side: Side,
    pub price: f64,
    pub amount: f64,

    pub success: bool, // TxConfirmed: true, TxFailed: false
    pub reason: Option<String>, // TxFailed 时记录原因
}

