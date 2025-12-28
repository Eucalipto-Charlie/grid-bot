use std::collections::VecDeque;
use rand::seq::SliceRandom;

use crate::execution::executor::Executor;
use crate::types::{
    intent::{TradeIntent, TradeResult},
    event::Event,
};

pub struct MockExecutor {
    //Strategy 刚 submit 的订单
    pending: VecDeque<TradeIntent>,

    //已被交易所接收（Submitted），但尚未成交
    submitted: VecDeque<TradeIntent>,

    //模拟网络 / 撮合延迟
    delayed: VecDeque<TradeIntent>,

    //用于控制行为节奏
    tick: u64,
}

impl MockExecutor {
    pub fn new() -> Self {
        Self {
            pending: VecDeque::new(),
            submitted: VecDeque::new(),
            delayed: VecDeque::new(),
            tick: 0,
        }
    }
}

impl Executor for MockExecutor {
    /// Strategy -> Executor
    fn submit(&mut self, intent: TradeIntent) {
        println!(
            "[Executor] submit order_id={} grid={} {:?}",
            intent.order_id,
            intent.grid_index,
            intent.side
        );

        // 只入队，不产生事件
        self.pending.push_back(intent);
    }

    /// Executor -> Strategy
    fn poll_events(&mut self) -> Vec<Event> {
        self.tick += 1;
        let mut events = Vec::new();
        let mut rng = rand::thread_rng();

        //pending -> submitted，必须产生 TxSubmitted
        if self.tick % 2 == 0 {
            if let Some(intent) = self.pending.pop_front() {
                let order_id = intent.order_id;

                println!(
                    "[Executor] order_id={} moved to Submitted",
                    order_id
                );

                self.submitted.push_back(intent);

                //通知 Strategy 状态迁移
                events.push(Event::TxSubmitted {
                    order_id,
                });
            }
        }

        //submitted -> delayed（制造乱序，不发事件）
        if self.tick % 3 == 0 {
            let mut batch: Vec<_> = self.submitted.drain(..).collect();
            batch.shuffle(&mut rng);

            for intent in batch {
                println!(
                    "[Executor] order_id={} moved to Delayed",
                    intent.order_id
                );
                self.delayed.push_back(intent);
            }
        }

        //delayed -> confirmed（最终成交）
        if self.tick % 4 == 0 {
            if let Some(intent) = self.delayed.pop_front() {
                println!(
                    "[Executor] delayed order_id={} confirmed (Filled)",
                    intent.order_id
                );

                events.push(Event::TxConfirmed {
                    order_id: intent.order_id,
                    result: TradeResult {
                        order_id: intent.order_id,
                        grid_index: intent.grid_index,
                        side: intent.side,
                        price: intent.price,
                        amount: intent.amount,
                        success: true,
                        reason: None,
                    },
                });
            }
        }

        events
    }
}

