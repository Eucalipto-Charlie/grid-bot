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
        }
    }

    pub fn handle_event(&mut self, event: Event) -> Vec<TradeIntent> {
        let mut intents = Vec::new();

        match event {
        Event::PriceCrossed { grid_index } => {
            if let Some(slot) = self.slots.get_mut(&grid_index) {
                if slot.state == GridState::WaitingBuy {
                    let price = self.grid.grid_price(grid_index);

                    slot.state = GridState::BuySubmitted;

                    intents.push(TradeIntent {
                        side: Side::Buy,
                        price,
                        amount: self.grid.amount_per_grid,
                    });
                }
            }
        }
        Event::TxConfirmed { result } => {
            match result.side {
                Side::Buy => {
                    // 找到对应买入 slot
                    for slot in self.slots.values_mut() {
                        if slot.state == GridState::BuySubmitted {
                            slot.state = GridState::WaitingSell;

                            let sell_price =
                                self.grid.grid_price(slot.index + 1);

                            intents.push(TradeIntent {
                                side: Side::Sell,
                                price: sell_price,
                                amount: result.amount,
                            });
                            break;
                        }
                    }
                }

                Side::Sell => {
                    for slot in self.slots.values_mut() {
                        if slot.state == GridState::WaitingSell {
                            slot.state = GridState::WaitingBuy;
                            break;
                        }
                    }
                }
            }
        }
        _ => {}
        }

        intents
    }
}