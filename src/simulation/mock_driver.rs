use crate::{
    strategy::{grid::GridConfig, state_machine::GridStateMachine},
    types::{
        event::Event,
        intent::{TradeResult, Side},
        grid_state::GridState,
    },
};

pub fn run_mock() {
    let grid = GridConfig {
        price_low: 90.0,
        price_high: 110.0,
        grid_count: 10,
        amount_per_grid: 1.0,
    };

    let mut sm = GridStateMachine::new(grid);

    // ===== 初始状态 =====
    println!("Initial state: {:?}", sm.state);
    assert_eq!(sm.state, GridState::Idle);

    // ===== Step 1：价格触发 =====
    let buy_intent = sm.handle_event(Event::PriceCrossed { grid_index: 3 });
    println!("After PriceCrossed:");
    println!("  State  = {:?}", sm.state);
    println!("  Intent = {:?}", buy_intent);

    assert!(buy_intent.is_some());
    assert_eq!(sm.state, GridState::BuySubmitted);

    let buy_intent = buy_intent.unwrap();

    // ===== Step 2：模拟 Buy 成交 =====
    let sell_intent = sm.handle_event(Event::TxConfirmed {
        result: TradeResult {
            side: Side::Buy,
            price: buy_intent.price,
            amount: buy_intent.amount,
        },
    });

    println!("After Buy TxConfirmed:");
    println!("  State  = {:?}", sm.state);
    println!("  Intent = {:?}", sell_intent);

    assert!(sell_intent.is_some());
    assert_eq!(sm.state, GridState::SellSubmitted);

    let sell_intent = sell_intent.unwrap();

    // ===== Step 3：模拟 Sell 成交（回补完成） =====
    let final_intent = sm.handle_event(Event::TxConfirmed {
        result: TradeResult {
            side: Side::Sell,
            price: sell_intent.price,
            amount: sell_intent.amount,
        },
    });

    println!("After Sell TxConfirmed:");
    println!("  State  = {:?}", sm.state);
    println!("  Intent = {:?}", final_intent);

    assert!(final_intent.is_none());
    assert_eq!(sm.state, GridState::WaitingBuy);

    println!("✅ Grid trading loop completed successfully");
}
