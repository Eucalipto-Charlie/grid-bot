#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GridState {
    Idle,

    WaitingBuy,
    BuySubmitted,
    BuyFilled,

    WaitingSell,
    SellSubmitted,
    SellFilled,

    Paused,
}
