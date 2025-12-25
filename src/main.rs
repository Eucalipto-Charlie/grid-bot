mod strategy;
mod types;
mod execution;
mod engine;
mod simulation;
mod market;

fn main() {
    simulation::scenario::run_basic_grid_scenario();
}
