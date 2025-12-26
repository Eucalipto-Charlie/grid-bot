use crate::{
    market::{price_feed::PriceFeed, grid_cross_detector::GridCrossDetector},
    execution::executor::Executor,
    strategy::state_machine::GridStateMachine,
    types::event::Event,
};

pub struct Engine<F, E> {
    feed: F,
    detector: GridCrossDetector,
    strategy: GridStateMachine,
    executor: E,
}

impl<F, E> Engine<F, E>
where
    F: PriceFeed,
    E: Executor,
{
    pub fn new(
        feed: F,
        detector: GridCrossDetector,
        strategy: GridStateMachine,
        executor: E,
    ) -> Self {
        Self {
            feed,
            detector,
            strategy,
            executor,
        }
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

        // 1. 拉价格 → PriceTick
        if let Some(price) = self.feed.poll_price() {
            events.push(Event::PriceTick { price });

            // 2. Detector 基于 price 生成 PriceCrossed
            let crossed = self.detector.on_price(price);
            for idx in crossed {
                events.push(Event::PriceCrossed { grid_index: idx });
            }
        }

        // 3. Executor 回执
        events.extend(self.executor.poll_events());

        // 4. 统一派发
        for event in events {
            let intents = self.strategy.handle_event(event);
            for intent in intents {
                self.executor.submit(intent);
            }
        }
    }

}
