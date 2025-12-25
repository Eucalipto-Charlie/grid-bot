use crate::types::grid_state::GridState;

#[derive(Debug)]
pub struct GridSlot {
    pub index: usize,
    pub state: GridState,
}

impl GridSlot {
    pub fn new(index: usize) -> Self {
        Self {
            index,
            state: GridState::WaitingBuy,
        }
    }
}
