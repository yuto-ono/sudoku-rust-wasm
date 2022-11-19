mod board;
mod constants;
use board::Board;
use constants::BOARD_NUM;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub enum SolveStatus {
    Success,
    Invalid,
    Duplicated,
    Unsolvable,
}

#[wasm_bindgen]
pub fn solve(num_array: &mut [u32]) -> SolveStatus {
    if num_array.len() != BOARD_NUM {
        return SolveStatus::Invalid;
    }

    let mut board = Board::new(&num_array);
    if !board.is_valid {
        return SolveStatus::Duplicated;
    }
    // TODO: 空きマス0の場合の例外処
    if !board.solve() {
        return SolveStatus::Unsolvable;
    }
    board.output_array(num_array);
    SolveStatus::Success
}
