use std::collections::HashMap;

use crate::types::{
    event::Event,
    intent::{TradeIntent, TradeResult, Side},
    grid_state::GridState,
};
use crate::strategy::{grid::GridConfig, grid_slot::GridSlot};

pub struct GridStateMachine {
    pub grid: GridConfig,
    pub slots: HashMap<usize, GridSlot>,
    pub orders: HashMap<u64, usize>,

    pub trade_intents: Vec<TradeIntent>,
    pub trade_results: Vec<TradeResult>,
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
            trade_intents: Vec::new(),
            trade_results: Vec::new(),
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

                        slot.transit(GridState::BuySubmitted);

                        // 记录 Buy 订单映射
                        self.orders.insert(order_id, grid_index);

                        let intent = TradeIntent {
                            order_id,
                            grid_index,
                            side: Side::Buy,
                            price,
                            amount: self.grid.amount_per_grid,
                        };

                        // 持久化 TradeIntent
                        self.trade_intents.push(intent.clone());

                        intents.push(intent);
                    }
                }
            }

            Event::TxConfirmed { order_id, result } => {
                if let Some(&grid_index) = self.orders.get(&order_id) {
                    self.orders.remove(&order_id);

                    if let Some(slot) = self.slots.get_mut(&grid_index) {
                        match result.side {
                            Side::Buy => {
                                slot.transit(GridState::BuyFilled);
                                slot.transit(GridState::WaitingSell);

                                let sell_price = self.grid.grid_price(grid_index + 1);
                                let new_order_id = rand::random::<u64>();

                                self.orders.insert(new_order_id, grid_index);

                                let intent = TradeIntent {
                                    order_id: new_order_id,
                                    grid_index,
                                    side: Side::Sell,
                                    price: sell_price,
                                    amount: result.amount,
                                };

                                // 持久化 TradeIntent
                                self.trade_intents.push(intent.clone());

                                intents.push(intent);
                            }

                            Side::Sell => {
                                slot.transit(GridState::SellFilled);
                                slot.transit(GridState::WaitingBuy);
                                // Sell 完成，不生成新订单
                            }
                        }

                        //持久化 TradeResult
                        self.trade_results.push(TradeResult {
                            order_id,
                            grid_index,
                            side: result.side,
                            price: result.price,
                            amount: result.amount,
                            success: true,
                            reason: None,
                        });
                    }
                }
            }

            Event::TxFailed { order_id, reason } => {
                if let Some(&grid_index) = self.orders.get(&order_id) {
                    self.orders.remove(&order_id);

                    if let Some(slot) = self.slots.get_mut(&grid_index) {
                        slot.transit(GridState::WaitingBuy);
                    }

                    // 持久化失败 TradeResult
                    self.trade_results.push(TradeResult {
                        order_id,
                        grid_index,
                        side: Side::Buy, // 从原始 TradeIntent 获取 side
                        price: 0.0,      // 可以记录失败时的预估价格
                        amount: 0.0,
                        success: false,
                        reason: Some(reason.clone()),
                    });

                    println!(
                        "[Strategy] Order {} failed on grid {}: {}",
                        order_id, grid_index, &reason
                    );
                }
            }

            _ => {}
        }

        intents
    }


    pub fn list_intents(&self) -> &[TradeIntent] {
        &self.trade_intents
    }

    pub fn list_results(&self) -> &[TradeResult] {
        &self.trade_results
    }

    pub fn last_result(&self) -> Option<&TradeResult> {
        self.trade_results.last()
    }

}
