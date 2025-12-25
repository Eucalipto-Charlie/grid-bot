pub trait PriceFeed {
    /// 获取最新价格（tick）
    fn poll_price(&mut self) -> Option<f64>;
}
