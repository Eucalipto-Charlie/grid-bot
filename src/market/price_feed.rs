pub trait PriceFeed {
    fn poll_price(&mut self) -> Option<f64>;
}

