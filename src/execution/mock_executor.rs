use std::collections::VecDeque;

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
        println!("[Executor] submit {:?}", intent);
        self.pending.push_back(intent);
    }

    fn poll_events(&mut self) -> Vec<Event> {
        let mut events = Vec::new();

        if let Some(intent) = self.pending.pop_front() {
            events.push(Event::TxConfirmed {
                result: TradeResult {
                    side: intent.side,
                    price: intent.price,
                    amount: intent.amount,
                },
            });
        }

        events
    }
}
