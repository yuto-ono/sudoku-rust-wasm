mod board;
pub mod constants;
use board::Board;
use constants::BOARD_NUM;
use wasm_bindgen::prelude::*;

pub enum SolveStatus {
    Success,
    InvalidLength,
    NoEmpty,
    Duplicated,
    Unsolvable,
}

#[wasm_bindgen]
pub fn solve(num_array: &mut [u32]) -> u32 {
    if num_array.len() != BOARD_NUM {
        return SolveStatus::InvalidLength as u32; // 配列の長さが違う
    }

    let mut board = Board::new(&num_array);
    if !board.is_valid {
        return SolveStatus::Duplicated as u32; // 重複がある
    }
    if board.empty_len == 0 {
        return SolveStatus::NoEmpty as u32; // 空きマスがない
    }
    if !board.solve() {
        return SolveStatus::Unsolvable as u32; // 解くことができない
    }
    board.output_array(num_array);
    SolveStatus::Success as u32 // 解けた
}
