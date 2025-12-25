use crate::{
    engine::event_loop::Engine,
    execution::mock_executor::MockExecutor,
    market::{
        mock_price_feed::MockPriceFeed,
        grid_cross_detector::GridCrossDetector,
    },
    strategy::{grid::GridConfig, state_machine::GridStateMachine},
};

pub fn run_basic_grid_scenario() {
    let grid = GridConfig {
        price_low: 90.0,
        price_high: 110.0,
        grid_count: 10,
        amount_per_grid: 1.0,
    };

    let strategy = GridStateMachine::new(grid.clone());
    let feed = MockPriceFeed::new(vec![95.0, 97.0, 99.0]);
    let detector = GridCrossDetector::new(grid);
    let executor = MockExecutor::new();

    let mut engine = Engine::new(
        feed,
        detector,
        strategy,
        executor,
    );

    for _ in 0..10 {
        engine.run();
    }

    println!("✅ basic grid scenario finished");
}
