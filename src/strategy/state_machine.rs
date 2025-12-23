use crate::types::{
    grid_state::GridState,
    event::Event,
    intent::{TradeIntent, Side},
};
use crate::strategy::grid::GridConfig;

pub struct GridStateMachine {
    pub state: GridState,
    pub grid: GridConfig,
    pub current_index: Option<usize>,
}

impl GridStateMachine {
    pub fn new(grid: GridConfig) -> Self {
        Self {
            state: GridState::Idle,
            grid,
            current_index: None,
        }
    }

    pub fn handle_event(&mut self, event: Event) -> Option<TradeIntent> {
        match (self.state, event) {

            (GridState::Idle, Event::PriceCrossed { grid_index }) => {
                let price = self.grid.grid_price(grid_index);
                self.state = GridState::BuySubmitted;
                self.current_index = Some(grid_index);

                Some(TradeIntent {
                    side: Side::Buy,
                    price,
                    amount: self.grid.amount_per_grid,
                })
            }

            (GridState::BuySubmitted, Event::TxConfirmed { result }) => {
                self.state = GridState::BuyFilled;

                let sell_index = self.current_index.unwrap() + 1;
                let price = self.grid.grid_price(sell_index);

                self.state = GridState::SellSubmitted;

                Some(TradeIntent {
                    side: Side::Sell,
                    price,
                    amount: result.amount,
                })
            }

            (GridState::SellSubmitted, Event::TxConfirmed { .. }) => {
                self.state = GridState::WaitingBuy;
                self.current_index = None;
                None
            }

            (_, Event::TxFailed { reason }) => {
                println!("Tx failed: {}", reason);
                self.state = GridState::Paused;
                None
            }

            _ => None,
        }
    }
}
