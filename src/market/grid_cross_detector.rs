use crate::strategy::grid::GridConfig;

pub struct GridCrossDetector {
    grid: GridConfig,
    last_price: Option<f64>,
}

impl GridCrossDetector {
    pub fn new(grid: GridConfig) -> Self {
        Self {
            grid,
            last_price: None,
        }
    }

    pub fn on_price(&mut self, price: f64) -> Vec<usize> {
        let mut crossed = Vec::new();

        if let Some(prev) = self.last_price {
            for i in 0..self.grid.grid_count {
                let gp = self.grid.grid_price(i);

                // 下穿网格
                if prev > gp && price <= gp {
                    crossed.push(i);
                }
            }
        }

        self.last_price = Some(price);
        crossed
    }
}
