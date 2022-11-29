pub mod constants;
use wasm_bindgen::prelude::*;
pub mod solver;

#[wasm_bindgen]
pub fn solve(num_array: &mut [u32]) -> u32 {
    solver::solve(num_array) as u32
}
