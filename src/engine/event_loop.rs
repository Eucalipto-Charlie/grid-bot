use crate::{
    market::{price_feed::PriceFeed, grid_cross_detector::GridCrossDetector},
    execution::executor::Executor,
    strategy::state_machine::GridStateMachine,
    types::event::Event,
    persistence::trade_store::TradeStore,
};

pub struct Engine<F, E, S> {
    feed: F,
    detector: GridCrossDetector,
    strategy: GridStateMachine,
    executor: E,
    store: S,
}

impl<F, E, S> Engine<F, E, S>
where
    F: PriceFeed,
    E: Executor,
    S: TradeStore,
{
    pub fn new(
        feed: F,
        detector: GridCrossDetector,
        strategy: GridStateMachine,
        executor: E,
        store: S,
    ) -> Self {
        Self {
            feed,
            detector,
            strategy,
            executor,
            store,
        }
    }

    //scenario 调试使用
    pub fn trade_store(&self) -> &S {
        &self.store
    }

    pub fn debug_snapshot(&self) {
        println!("--- Engine Snapshot ---");

        for (idx, slot) in self.strategy.slots.iter() {
            println!("Grid {} => {:?}", idx, slot.state);
        }

        println!("Orders mapping: {:?}", self.strategy.orders);
        println!("-----------------------");
    }

    pub fn run(&mut self) {
        let mut events = Vec::new();

        // 1. Price
        if let Some(price) = self.feed.poll_price() {
            events.push(Event::PriceTick { price });

            let crossed = self.detector.on_price(price);
            for idx in crossed {
                events.push(Event::PriceCrossed { grid_index: idx });
            }
        }

        // 2. Executor events
        events.extend(self.executor.poll_events());

        // 3. Dispatch
        for event in events {
            let prev_result_len = self.strategy.trade_results.len();

            let intents = self.strategy.handle_event(event);

            // 持久化 TradeIntent
            for intent in intents {
                self.store.record_intent(&intent);
                self.executor.submit(intent);
            }

            let new_results = &self.strategy.trade_results[prev_result_len..];
            for result in new_results {
                self.store.record_result(result);
            }
        }
    }
}
