use std::collections::HashMap;

use crate::types::{
    event::Event,
    intent::{TradeIntent, Side},
    grid_state::GridState,
};
use crate::strategy::{grid::GridConfig, grid_slot::GridSlot};

pub struct GridStateMachine {
    pub grid: GridConfig,
    pub slots: HashMap<usize, GridSlot>,
    pub orders: HashMap<u64, usize>,
}

impl GridStateMachine {
    pub fn new(grid: GridConfig) -> Self {
        let mut slots = HashMap::new();

        for i in 0..grid.grid_count {
            slots.insert(i, GridSlot::new(i));
        }

        Self {
            grid,
            slots,
            orders: HashMap::new(),
        }
    }

    pub fn handle_event(&mut self, event: Event) -> Vec<TradeIntent> {
        let mut intents = Vec::new();

        match event {
        Event::PriceCrossed { grid_index } => {
            if let Some(slot) = self.slots.get_mut(&grid_index) {
                if slot.state == GridState::WaitingBuy {
                    let price = self.grid.grid_price(grid_index);
                    let order_id = rand::random::<u64>();

                    slot.state = GridState::BuySubmitted;

                    // 关键：记录映射
                    self.orders.insert(order_id, grid_index);

                    intents.push(TradeIntent {
                        order_id,
                        grid_index,
                        side: Side::Buy,
                        price,
                        amount: self.grid.amount_per_grid,
                    });
                }
            }
        }
        Event::TxConfirmed { order_id, result } => {
            // 用 order_id 精确定位 grid
            if let Some(&grid_index) = self.orders.get(&order_id) {
                let slot = self.slots.get_mut(&grid_index).unwrap();

                match result.side {
                    Side::Buy => {
                        slot.state = GridState::WaitingSell;

                        let sell_price = self.grid.grid_price(grid_index + 1);
                        let new_order_id = rand::random::<u64>();

                        // 记录新的 Sell 订单
                        self.orders.insert(new_order_id, grid_index);

                        intents.push(TradeIntent {
                            order_id: new_order_id,
                            grid_index,
                            side: Side::Sell,
                            price: sell_price,
                            amount: result.amount,
                        });
                    }

                    Side::Sell => {
                        slot.state = GridState::WaitingBuy;

                        // Sell 完成，清理映射
                        self.orders.remove(&order_id);
                    }
                }
            }
        }

        _ => {}
        }

        intents
    }
}