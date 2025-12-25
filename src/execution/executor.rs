use crate::types::{intent::TradeIntent, event::Event};

/// Executor 抽象
/// 负责接收策略意图，并输出链上已确认事件
pub trait Executor {
    /// 提交交易 Intent 到执行层（可能是 mock 或真实链）
    fn submit(&mut self, intent: TradeIntent);

    /// 从执行层获取链上事件（已确认成交等）
    fn poll_events(&mut self) -> Vec<Event>;
}
