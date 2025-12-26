use std::collections::VecDeque;
use rand::seq::SliceRandom;

use crate::execution::executor::Executor;
use crate::types::{
    intent::{TradeIntent, Side},
    event::{Event},
    intent::TradeResult,
};

pub struct MockExecutor {
    pending: VecDeque<TradeIntent>,
}

impl MockExecutor {
    pub fn new() -> Self {
        Self {
            pending: VecDeque::new(),
        }
    }
}

impl Executor for MockExecutor {
    fn submit(&mut self, intent: TradeIntent) {
        println!(
            "[Executor] submit order_id={} grid={} {:?}",
            intent.order_id,
            intent.grid_index,
            intent
        );
        self.pending.push_back(intent);
    }

    fn poll_events(&mut self) -> Vec<Event> {
        let mut events = Vec::new();

        let mut batch = Vec::new();
        while let Some(intent) = self.pending.pop_front() {
            batch.push(intent);
        }

        // 随机打乱顺序
        let mut rng = rand::thread_rng();
        batch.shuffle(&mut rng);

        for intent in batch {
            events.push(Event::TxConfirmed {
                order_id: intent.order_id,
                result: TradeResult {
                    order_id: intent.order_id,
                    grid_index: intent.grid_index,
                    side: intent.side,
                    price: intent.price,
                    amount: intent.amount,
                },
            });
        }

        events
    }

}
