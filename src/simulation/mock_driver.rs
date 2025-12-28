use crate::{
    market::{mock_price_feed::MockPriceFeed, grid_cross_detector::GridCrossDetector},
    execution::mock_executor::MockExecutor,
    strategy::{grid::GridConfig, state_machine::GridStateMachine},
    engine::event_loop::Engine,
};

pub fn run_mock() {
    let grid = GridConfig {
        price_low: 90.0,
        price_high: 110.0,
        grid_count: 10,
        amount_per_grid: 1.0,
    };

    let feed = MockPriceFeed::new(vec![
        100.0,
        97.0,
        95.0, 
        99.0, 
    ]);

    let detector = GridCrossDetector::new(grid.clone());
    let strategy = GridStateMachine::new(grid);
    let executor = MockExecutor::new();

    let mut engine = Engine::new(
        feed, 
        detector, 
        strategy, 
        executor
    );

    for _ in 0..10 {
        engine.run();
    }
}

