#[derive(Debug, Clone, Copy)]
pub enum Side {
    Buy,
    Sell,
}

#[derive(Debug)]
pub struct TradeIntent {
    pub side: Side,
    pub price: f64,
    pub amount: f64,
}

#[derive(Debug)]
pub struct TradeResult {
    pub side: Side,
    pub price: f64,
    pub amount: f64,
}
