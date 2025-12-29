use crate::{
    engine::event_loop::Engine,
    execution::mock_executor::MockExecutor,
    market::{
        mock_price_feed::MockPriceFeed,
        grid_cross_detector::GridCrossDetector,
    },
    strategy::{grid::GridConfig, state_machine::GridStateMachine},
    persistence::hybrid_store::HybridTradeStore, 
};

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

    // 内存调试和文件持久化
    let store = HybridTradeStore::new("./data");

    let executor = MockExecutor::new();

    let mut engine = Engine::new(feed, detector, strategy, executor, store);

    for round in 0..20 {
        println!("\n=== Engine run round {} ===", round + 1);
        engine.run();
        engine.debug_snapshot();
    }

    // 内存 dump 用于表格输出
    engine.trade_store().dump();
    engine.trade_store().dump_table();

    println!("\nbasic grid scenario finished!");
}
