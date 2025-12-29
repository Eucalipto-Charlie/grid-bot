/// GridState 描述单个网格（GridSlot）在策略中的交易生命周期状态。
///
/// # 状态机模型（Finite State Machine）
///
/// 每个 GridSlot 被建模为一个**严格单向的有限状态机（FSM）**，
/// 同一时间最多只允许存在一个活跃订单。
///
///
/// ## 初始状态
///
/// - 当前策略实现中，GridSlot 初始化后直接进入 `WaitingBuy`
/// - `Idle` 与 `Paused` 为**预留状态**，用于未来扩展（例如风控 / 手动干预）
///
///
/// ## 主交易路径（当前启用）
///
/// ```text
/// WaitingBuy
///     └── submit Buy TradeIntent
///         → BuySubmitted
///             └── Buy 成交确认
///                 → BuyFilled
///                     → WaitingSell
///                         └── submit Sell TradeIntent
///                             → SellSubmitted
///                                 └── Sell 成交确认
///                                     → SellFilled
///                                         → WaitingBuy
/// ```
///
///
/// ## 设计约束
///
/// - 状态流转是**单向的，不允许回退**
/// - 不允许跳跃中间状态（例如 WaitingBuy → WaitingSell）
/// - 每一次状态变化，必须由明确事件驱动（Intent / Result）
///
///
/// ## 预留状态说明
///
/// - `Idle`：
///   - 表示网格未参与交易（系统启动前 / 尚未激活）
///
/// - `Paused`：
///   - 表示网格被主动暂停（风控、人工干预、异常处理）
///
/// 这些状态当前未被构造，但作为系统演进的扩展点被保留。
///
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GridState {
    // 未激活状态（预留）
    Idle,

    // 等待提交 Buy 订单
    WaitingBuy,

    // Buy 订单已提交，等待成交
    BuySubmitted,

    // Buy 已成交，准备进入卖出阶段
    BuyFilled,

    // 等待提交 Sell 订单
    WaitingSell,

    // Sell 订单已提交，等待成交
    SellSubmitted,

    // Sell 已成交，一个完整 grid cycle 结束
    SellFilled,

    // 网格被暂停（预留）
    Paused,
}

impl Default for GridState {
    fn default() -> Self {
        GridState::WaitingBuy
    }
}

impl GridState {
    /// 判断当前状态是否允许迁移到 next 状态
    pub fn can_transit_to(self, next: GridState) -> bool {
        use GridState::*;

        matches!(
            (self, next),
            // 初始化
            (Idle, WaitingBuy)

            // Buy 流程
            | (WaitingBuy, BuySubmitted)
            | (BuySubmitted, BuyFilled)

            // Sell 流程
            | (BuyFilled, WaitingSell)
            | (WaitingSell, SellSubmitted)
            | (WaitingSell, SellFilled)
            | (SellSubmitted, SellFilled)

            // 完成一次完整循环
            | (SellFilled, WaitingBuy)

            // 任意状态可被暂停
            | (_, Paused)
        )
    }
}
