use crate::market::price_feed::PriceFeed;

pub struct MockPriceFeed {
    prices: Vec<f64>,
    cursor: usize,
}

impl MockPriceFeed {
    pub fn new(prices: Vec<f64>) -> Self {
        Self { prices, cursor: 0 }
    }
}

impl PriceFeed for MockPriceFeed {
    fn poll_price(&mut self) -> Option<f64> {
        if self.cursor >= self.prices.len() {
            None
        } else {
            let p = self.prices[self.cursor];
            self.cursor += 1;
            Some(p)
        }
    }
}
