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
            state: GridState::default(),
        }
    }

    pub fn transit(&mut self, next: GridState) {
        debug_assert!(
            self.state.can_transit_to(next),
            "Illegal GridState transition: {:?} -> {:?}",
            self.state,
            next
        );

        self.state = next;
    }
}
