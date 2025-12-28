use crate::{
    engine::event_loop::Engine,
    execution::mock_executor::MockExecutor,
    market::{
        mock_price_feed::MockPriceFeed,
        grid_cross_detector::GridCrossDetector,
    },
    strategy::{grid::GridConfig, state_machine::GridStateMachine},
};

use rand::seq::SliceRandom;

pub fn run_basic_grid_scenario() {
    let grid = GridConfig {
        price_low: 90.0,
        price_high: 110.0,
        grid_count: 10,
        amount_per_grid: 1.0,
    };

    let strategy = GridStateMachine::new(grid.clone());
    let feed = MockPriceFeed::new(vec![100.0, 97.0, 95.0, 99.0]);
    let detector = GridCrossDetector::new(grid);

    // 支持乱序 order_id 的 MockExecutor
    let executor = MockExecutor::new();

    let mut engine = Engine::new(feed, detector, strategy, executor);

    // 运行多轮，模拟真实引擎 loop
    for round in 0..10 {
        println!("\n=== Engine run round {} ===", round + 1);

        //Engine 执行一轮
        engine.run();

        //通过 Engine 提供的调试接口观察内部状态
        engine.debug_snapshot();
    }

    println!("\nbasic grid scenario finished (乱序验证)");
}


