pub struct GridConfig {
    pub price_low: f64,
    pub price_high: f64,
    pub grid_count: usize,
    pub amount_per_grid: f64,
}

impl GridConfig {
    pub fn grid_price(&self, index: usize) -> f64 {
        let step = (self.price_high - self.price_low) / self.grid_count as f64;
        self.price_low + step * index as f64
    }
}
