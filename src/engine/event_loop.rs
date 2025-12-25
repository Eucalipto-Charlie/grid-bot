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

    pub fn run(&mut self) {
        // 1. 拉价格
        if let Some(price) = self.feed.poll_price() {
            // 2. 检测跨格
            let crossed = self.detector.on_price(price);

            for idx in crossed {
                let intents = self.strategy.handle_event(
                    Event::PriceCrossed { grid_index: idx },
                );

                for intent in intents {
                    self.executor.submit(intent);
                }
            }
        }

        // 3. 拉执行结果
        for event in self.executor.poll_events() {
            let intents = self.strategy.handle_event(event);
            for intent in intents {
                self.executor.submit(intent);
            }
        }
    }
}
